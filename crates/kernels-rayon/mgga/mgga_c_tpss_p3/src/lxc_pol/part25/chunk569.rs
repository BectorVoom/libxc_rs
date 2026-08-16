//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 569/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk569(t1068: f64, t294: f64, t215: f64, t442: f64, t671: f64, t441: f64, t1102: f64, t140: f64, t1098: f64, t1014: f64, t390: f64, t2840: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3009 = t294 * t1068;
    let t3025 = t215 * t671 * t442;
    let t3027 = t441 * t3025 / 432.0_f64;
    let t3028 = t140 * t1102;
    let t3029 = t1098 * t3028;
    let t3032 = 1.0_f64 / t390 / t1014;
    let t3033 = t3032 * t2840;
    (t3009, t3025, t3027, t3028, t3029, t3032, t3033)
}
