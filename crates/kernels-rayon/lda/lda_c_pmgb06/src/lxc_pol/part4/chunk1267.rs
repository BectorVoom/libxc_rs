//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1267/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1267(t15382: f64, t439: f64, t5260: f64, t12154: f64, t15387: f64, t5168: f64, t6478: f64, t2010: f64, t5253: f64, t6155: f64, t1420: f64, t6416: f64) -> (f64, f64, f64, f64, f64) {
    let t16649 = 8.0_f64 / 81.0_f64 * t439 * t5260 * t15382;
    let t16652 = 88.0_f64 / 243.0_f64 * t439 * t12154 * t15387;
    let t16654 = 8.0_f64 / 27.0_f64 * t5168 * t6478;
    let t16657 = 8.0_f64 / 27.0_f64 * t2010 * t5253 * t6155;
    let t16659 = 4.0_f64 / 45.0_f64 * t1420 * t6416;
    (t16649, t16652, t16654, t16657, t16659)
}
