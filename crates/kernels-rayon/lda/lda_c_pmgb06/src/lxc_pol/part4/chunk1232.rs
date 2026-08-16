//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1232/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1232(t1069: f64, t1438: f64, t2604: f64, t439: f64, t9084: f64, t1074: f64, t2864: f64, t6522: f64, t432: f64, t6836: f64, t132: f64, t435: f64, t6674: f64) -> (f64, f64, f64, f64) {
    let t16228 = 2.0_f64 / 27.0_f64 * t439 * t9084 * t2604 * t1438 * t1069;
    let t16237 = 2.0_f64 / 45.0_f64 * t439 * t2864 * t6522 * t1074;
    let t16238 = t432 * t6836;
    let t16239 = 2.0_f64 / 45.0_f64 * t16238;
    let t16241 = t132 * t435 * t6674;
    (t16228, t16237, t16239, t16241)
}
