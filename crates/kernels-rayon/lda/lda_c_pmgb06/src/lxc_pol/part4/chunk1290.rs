//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1290/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1290(t1416: f64, t493: f64, t6130: f64, t1417: f64, t6134: f64, t1972: f64, t4757: f64, t1559: f64, t439: f64, t6123: f64, t1560: f64, t6127: f64) -> (f64, f64, f64, f64, f64) {
    let t16952 = 2.0_f64 / 45.0_f64 * t493 * t6130 * t1416;
    let t16954 = 2.0_f64 / 45.0_f64 * t6134 * t1417;
    let t16956 = 4.0_f64 / 45.0_f64 * t1972 * t4757;
    let t16959 = 2.0_f64 / 45.0_f64 * t439 * t6123 * t1559;
    let t16961 = 2.0_f64 / 45.0_f64 * t6127 * t1560;
    (t16952, t16954, t16956, t16959, t16961)
}
