//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1234/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1234(t1995: f64, t5194: f64, t1629: f64, t1966: f64, t439: f64, t6554: f64, t493: f64, t5175: f64, t6119: f64, t12239: f64, t12241: f64, t432: f64, t6736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16254 = t5194 * t1995;
    let t16255 = 8.0_f64 / 45.0_f64 * t16254;
    let t16259 = t439 * t1966 * t6554 * t1629 / 15.0_f64;
    let t16262 = 4.0_f64 / 15.0_f64 * t493 * t6119 * t5175;
    let t16263 = 4.0_f64 / 135.0_f64 * t12239;
    let t16264 = 4.0_f64 / 45.0_f64 * t12241;
    let t16266 = t432 * t6736 / 15.0_f64;
    (t16255, t16259, t16262, t16263, t16264, t16266)
}
