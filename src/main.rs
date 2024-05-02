use anyhow::{self, Ok, Result};
use esp_idf_hal::{
    gpio::{IOPin, PinDriver},
    peripherals::Peripherals,
    delay::FreeRtos,
};
use std::{thread::sleep, time::Duration};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;
// use esp_idf_svc::wifi::{Configuration,ClientConfiguration};
// use esp_idf_svc::wifi::AuthMethod;
use heapless::String as Hstring;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig,EspHttpServer};
use esp_idf_svc::http::Method;
fn main() ->Result<()>{
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    //WIFI CONFIG
    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;
    let ssid: Hstring<32> = Hstring::try_from(env!("WIFI_SSID")).unwrap();
    let password: Hstring<64> = Hstring::try_from(env!("WIFI_PASSWORD")).unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid,
        password,
        bssid:None,
        auth_method:AuthMethod::None,
        ..Default::default()
    }))?;
    wifi.start()?;
    wifi.connect()?;
    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default())?;
    httpserver.fn_handler("/", Method::Get, | request| {
        // Retrieve html String
        let html = index_html();
        // Respond with OK status
        let mut response = request.into_ok_response()?;
        // Return Requested Object (Index Page)
        response.write(html.as_bytes())?;
        Ok(())
    })?;
  
    if let core::result::Result::Ok(_)=wifi.is_connected(){
        loop {
            sleep(Duration::from_millis(1000));
        }
    }
    
    Ok(())
}


fn index_html() ->String {
    format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <title>esp-rs web server</title>
            </head>
            <body>
            Hello World from ESP!
            </body>
        </html>
        "#
    )
}