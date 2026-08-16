//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1251/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1251(t432: f64, t6626: f64, t1604: f64, t2563: f64, t1600: f64, t2553: f64, t1602: f64, t161: f64, t166: f64, t132: f64, t137: f64, t2106: f64, t5039: f64) -> (f64, f64, f64, f64) {
    let t16455 = t432 * t6626;
    let t16456 = 4.0_f64 / 45.0_f64 * t16455;
    let t16458 = t2563 * t1604 / 15.0_f64;
    let t16459 = t2553 * t1600;
    let t16463 = t161 * t166 * t16459 * t1602 / 15.0_f64;
    let t16467 = t132 * t137 * t2106 * t5039 / 15.0_f64;
    (t16456, t16458, t16463, t16467)
}
