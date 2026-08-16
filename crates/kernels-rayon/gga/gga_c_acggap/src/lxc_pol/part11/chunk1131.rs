//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1131/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1131(t31285: f64, t4360: f64, t7741: f64, t13287: f64, t34823: f64, t34828: f64, t31312: f64, t31316: f64, t31322: f64, t31284: f64, t31287: f64, t31291: f64, t31293: f64, t31296: f64, t31297: f64, t31299: f64, t31305: f64, t31318: f64, t31341: f64, t31342: f64, t31344: f64) -> f64 {
    let t35527 = 0.10718504529517434243e-2_f64 * t31285;
    let t35529 = t7741 * t4360;
    let t35535 = t34823 * t13287 * t34828;
    let t35538 = 0.85748036236139473944e-3_f64 * t31312;
    let t35539 = 0.12579236915841660827e-2_f64 * t31316;
    let t35541 = 0.85748036236139473944e-3_f64 * t31322;
    let t35544 = -t31284 - t35527 - 0.68598428988911579156e-2_f64 * t31287 + t31291 - 0.34299214494455789578e-2_f64 * t35529 + t31293 / 32.0_f64 - t31296 - 0.31448092289604152068e-2_f64 * t31297 + 0.28303283060643736861e-1_f64 * t31299 - 0.85748036236139473944e-3_f64 * t35535 - 0.40015750243531754508e-2_f64 * t31305 - t35538 + t35539 + 0.11321313224257494744e-1_f64 * t31318 + t35541 + t31341 + 7.0_f64 / 144.0_f64 * t31342 + 7.0_f64 / 288.0_f64 * t31344;
    t35544
}
