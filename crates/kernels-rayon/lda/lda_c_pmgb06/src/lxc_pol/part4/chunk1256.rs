//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1256/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1256(t1387: f64, t6127: f64, t493: f64, t5486: f64, t5493: f64, t1447: f64, t6509: f64, t5499: f64, t6513: f64, t332: f64, t477: f64, t6637: f64) -> (f64, f64, f64, f64, f64) {
    let t16518 = 2.0_f64 / 45.0_f64 * t6127 * t1387;
    let t16521 = 4.0_f64 / 45.0_f64 * t493 * t5486 * t5493;
    let t16522 = t1447 * t6509;
    let t16523 = 32.0_f64 / 243.0_f64 * t16522;
    let t16524 = t5499 * t6513;
    let t16525 = 20.0_f64 / 81.0_f64 * t16524;
    let t16527 = t6637 * t477 * t332;
    (t16518, t16521, t16523, t16525, t16527)
}
