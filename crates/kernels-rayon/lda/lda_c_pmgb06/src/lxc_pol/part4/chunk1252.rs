//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1252/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1252(t9619: f64, t153: f64, t1864: f64, t439: f64, t4779: f64, t4672: f64, t6494: f64, t4650: f64, t6498: f64, t2010: f64, t4668: f64, t1420: f64, t6499: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16468 = 2.0_f64 / 135.0_f64 * t9619;
    let t16472 = 8.0_f64 / 45.0_f64 * t439 * t4779 * t153 * t1864;
    let t16475 = 4.0_f64 / 45.0_f64 * t439 * t6494 * t4672;
    let t16478 = 4.0_f64 / 9.0_f64 * t439 * t6498 * t4650;
    let t16481 = 16.0_f64 / 45.0_f64 * t2010 * t6494 * t4668;
    let t16483 = 4.0_f64 / 27.0_f64 * t1420 * t6499;
    (t16468, t16472, t16475, t16478, t16481, t16483)
}
