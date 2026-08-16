//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1110/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1110(t15573: f64, t15587: f64, t12136: f64, t6759: f64, t6763: f64, t6767: f64, t2337: f64, t811: f64, t3974: f64, t3976: f64, t593: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20688 = 16.0_f64 / 45.0_f64 * t15573;
    let t20689 = 8.0_f64 / 45.0_f64 * t15587;
    let t20691 = 16.0_f64 / 15.0_f64 * t12136 * t6759;
    let t20693 = 32.0_f64 / 15.0_f64 * t12136 * t6763;
    let t20695 = 16.0_f64 / 9.0_f64 * t12136 * t6767;
    let t20696 = t2337 * t811;
    let t20700 = 8.0_f64 / 15.0_f64 * t3974 * t3976 * t20696 * t593;
    let t20701 = t20696 * t352;
    (t20688, t20689, t20691, t20693, t20695, t20700, t20701)
}
