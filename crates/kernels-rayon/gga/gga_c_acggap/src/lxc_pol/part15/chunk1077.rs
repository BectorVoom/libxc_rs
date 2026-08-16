//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1077/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1077(t36429: f64, t38383: f64, t7963: f64, t4241: f64, t7942: f64, t32130: f64, t38052: f64, t7965: f64, t2387: f64, t848: f64, t5351: f64, t8347: f64) -> (f64, f64, f64, f64, f64) {
    let t38386 = 0.34694512752820797848e1_f64 * t7963 * t38383 * t36429;
    let t38389 = 0.34694512752820797848e1_f64 * t7942 * t38383 * t4241;
    let t38392 = 0.34694512752820797848e1_f64 * t32130 * t38052 * t7965;
    let t38393 = t848 * t2387;
    let t38397 = t8347 * t5351;
    (t38386, t38389, t38392, t38393, t38397)
}
