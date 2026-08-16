//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 497/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk497(t3356: f64, t406: f64, t281: f64, t2902: f64, t414: f64, t1146: f64, t698: f64, t1224: f64, t240: f64, t1129: f64, t408: f64, t421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3402 = 0.39862222222222222223e0_f64 * t3356;
    let t3407 = 1.0_f64/f64::sqrt(t406);
    let t3413 = t281 * t2902 * t414;
    let t3414 = 0.13692777777777777778e0_f64 * t3413;
    let t3415 = t698 * t1146;
    let t3417 = t240 * t1224;
    let t3431 = t1129 * t1129;
    let t3432 = 1.0_f64 / t3431;
    let t3433 = t408 * t3432;
    let t3434 = t421 * t421;
    (t3402, t3407, t3413, t3414, t3415, t3417, t3433, t3434)
}
