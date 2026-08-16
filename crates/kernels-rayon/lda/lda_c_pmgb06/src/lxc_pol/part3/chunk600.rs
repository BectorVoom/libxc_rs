//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 600/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk600(t1992: f64, t3284: f64, t493: f64, t1444: f64, t1455: f64, t1592: f64, t458: f64, t1594: f64, t137: f64, t132: f64, t1595: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3285 = t1992 * t3284;
    let t3287 = t493 * t3285 / 5.0_f64;
    let t3289 = t1444 * t1455 / 15.0_f64;
    let t3290 = t458 * t1592;
    let t3291 = t3290 * t1594;
    let t3292 = t137 * t3291;
    let t3294 = t132 * t3292 / 5.0_f64;
    let t3295 = t435 * t1595;
    (t3285, t3287, t3289, t3290, t3291, t3292, t3294, t3295)
}
