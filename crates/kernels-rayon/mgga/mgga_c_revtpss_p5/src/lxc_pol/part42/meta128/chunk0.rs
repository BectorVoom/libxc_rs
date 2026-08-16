//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 625/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk625(t1126: f64, t1130: f64, t1129: f64, t418: f64, t408: f64, t406: f64, t409: f64, t3356: f64, t281: f64, t2902: f64, t414: f64, t1146: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3379 = t1126 * t1130;
    let t3382 = t1129 * t418;
    let t3383 = 1.0_f64 / t3382;
    let t3384 = t408 * t3383;
    let t3390 = 1.0_f64 / t409 / t406;
    let t3394 = 4.0_f64 / 9.0_f64 * t3356;
    let t3402 = 0.39862222222222222223e0_f64 * t3356;
    let t3407 = 1.0_f64/f64::sqrt(t406);
    let t3413 = t281 * t2902 * t414;
    let t3414 = 0.13692777777777777778e0_f64 * t3413;
    let t3415 = t698 * t1146;
    (t3379, t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3415)
}
