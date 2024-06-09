use opentelemetry::trace::Tracer as _;

fn main() {
    env_logger::init();

    let exporter =
        opentelemetry_application_insights::new_exporter_from_env(reqwest::blocking::Client::new())
            .expect("env var APPLICATIONINSIGHTS_CONNECTION_STRING should exist");
    let tracer = opentelemetry_application_insights::new_pipeline(exporter)
        .traces()
        .install_simple();

    tracer.in_span("reqwest-blocking-client", |_cx| {});

    opentelemetry::global::shutdown_tracer_provider();
}
