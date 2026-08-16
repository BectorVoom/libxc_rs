//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 599/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk599(t1420: f64, t1441: f64, t1423: f64, t1431: f64, t1426: f64, t1430: f64, t439: f64, t1435: f64, t458: f64, t1440: f64, t1586: f64, t1600: f64, t529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3271 = t1420 * t1441 / 9.0_f64;
    let t3272 = t1423 * t1431;
    let t3273 = 2.0_f64 / 45.0_f64 * t3272;
    let t3274 = t1423 * t1441;
    let t3275 = 2.0_f64 / 27.0_f64 * t3274;
    let t3276 = t1426 * t1430;
    let t3278 = t439 * t3276 / 15.0_f64;
    let t3279 = t1435 * t458;
    let t3280 = t3279 * t1440;
    let t3282 = t439 * t3280 / 9.0_f64;
    let t3284 = t1600 * t1586 * t529;
    (t3271, t3272, t3273, t3274, t3275, t3276, t3278, t3279, t3280, t3282, t3284)
}
