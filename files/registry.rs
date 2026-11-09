//! Windows registry read helpers for U He Tyrell N6.

pub const APP_KEY: &str = "Software\\U\\U He Tyrell N6";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\U He Tyrell N6";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\U He Tyrell N6") },
        RegValue { name: "Version",     value: String::from("v3.3.20") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("U He Tyrell N6") },
        RegValue { name: "Publisher",       value: String::from("U") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\U He Tyrell N6\\uninstall.exe\"") },
    ]
}
