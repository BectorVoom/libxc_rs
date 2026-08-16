//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 801/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk801(t1915: f64, t7512: f64, t493: f64, t6764: f64, t764: f64, t1919: f64, t2541: f64, t851: f64, t2991: f64, t6773: f64, t760: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7513 = t1915 * t7512;
    let t7515 = 2.0_f64 / 15.0_f64 * t493 * t7513;
    let t7516 = t6764 * t764;
    let t7517 = t1919 * t7516;
    let t7519 = t493 * t7517 / 9.0_f64;
    let t7520 = t2541 * t851;
    let t7521 = t2991 * t7520;
    let t7523 = t493 * t7521 / 9.0_f64;
    let t7524 = t6773 * t760;
    let t7525 = t1385 * t7524;
    (t7513, t7515, t7516, t7517, t7519, t7520, t7521, t7523, t7524, t7525)
}
