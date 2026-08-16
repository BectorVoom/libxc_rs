//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1262/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1262(t32923: f64, t36127: f64, t36129: f64, t36135: f64, t36137: f64, t36139: f64, t36141: f64, t37872: f64, t37875: f64, t37876: f64, t37888: f64, t37892: f64, t40425: f64, t40427: f64, t40431: f64, t40436: f64, t40442: f64, t40446: f64) -> f64 {
    let t42110 = -t40425 / 96.0_f64 + t37872 + 0.56606566121287473722e-1_f64 * t40427 + 0.31448092289604152069e-2_f64 * t40431 + 0.15095084299009992993e-1_f64 * t36127 - 0.85748036236139473944e-3_f64 * t36129 - t37875 - t37876 - 0.11433071498151929859e-2_f64 * t36135 - 0.12579236915841660828e-2_f64 * t40436 + 0.79249192569802463215e-1_f64 * t36137 - 0.64025200389650807212e-1_f64 * t36139 - t32923 - t36141 - 0.42874018118069736972e-2_f64 * t40442 + t37888 - t40446 / 64.0_f64 - t37892;
    t42110
}
