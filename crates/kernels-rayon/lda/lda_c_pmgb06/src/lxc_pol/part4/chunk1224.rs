//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1224/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1224(t1972: f64, t5180: f64, t136: f64, t1872: f64, t1968: f64, t439: f64, t4762: f64, t4608: f64, t6550: f64, t12075: f64, t1423: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16126 = 4.0_f64 / 15.0_f64 * t1972 * t5180;
    let t16130 = 4.0_f64 / 15.0_f64 * t439 * t136 * t1872 * t1968;
    let t16132 = 2.0_f64 / 5.0_f64 * t1972 * t4762;
    let t16135 = 2.0_f64 / 15.0_f64 * t439 * t6550 * t4608;
    let t16136 = 4.0_f64 / 15.0_f64 * t12075;
    let t16137 = t1423 * t6556;
    (t16126, t16130, t16132, t16135, t16136, t16137)
}
