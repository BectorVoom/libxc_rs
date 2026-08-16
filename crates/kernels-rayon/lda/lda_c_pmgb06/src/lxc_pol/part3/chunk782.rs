//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 782/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk782(t1080: f64, t1414: f64, t851: f64, t1380: f64, t493: f64, t1423: f64, t1894: f64, t1594: f64, t809: f64, t2864: f64, t439: f64, t2022: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5358 = t851 * t1414 * t1080;
    let t5359 = t1380 * t5358;
    let t5361 = 2.0_f64 / 45.0_f64 * t493 * t5359;
    let t5363 = 4.0_f64 / 135.0_f64 * t1423 * t1894;
    let t5364 = t809 * t1594;
    let t5365 = t2864 * t5364;
    let t5367 = 2.0_f64 / 45.0_f64 * t439 * t5365;
    let t5369 = 4.0_f64 / 9.0_f64 * t2022 * t591;
    (t5358, t5359, t5361, t5363, t5364, t5365, t5367, t5369)
}
