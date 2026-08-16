//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 548/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk548(t3236: f64, t407: f64, t281: f64, t2820: f64, t415: f64, t1114: f64, t699: f64, t1176: f64, t241: f64, t1097: f64, t409: f64, t422: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3282 = 0.39862222222222222223e0_f64 * t3236;
    let t3287 = 1.0_f64/f64::sqrt(t407);
    let t3293 = t281 * t2820 * t415;
    let t3294 = 0.13692777777777777778e0_f64 * t3293;
    let t3295 = t699 * t1114;
    let t3297 = t241 * t1176;
    let t3311 = t1097 * t1097;
    let t3312 = 1.0_f64 / t3311;
    let t3313 = t409 * t3312;
    let t3314 = t422 * t422;
    (t3282, t3287, t3293, t3294, t3295, t3297, t3313, t3314)
}
