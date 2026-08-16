//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1179/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1179(t21397: f64, t542: f64, t12322: f64, t4488: f64, t12380: f64, t21410: f64, t12362: f64, t21414: f64, t4501: f64, t20733: f64, t4494: f64, t12387: f64, t20737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21468 = t21397 * t542;
    let t21471 = 8.0_f64 / 9.0_f64 * t4488 * t12322 * t21468;
    let t21474 = 32.0_f64 / 27.0_f64 * t4488 * t12380 * t21410;
    let t21477 = 16.0_f64 / 9.0_f64 * t12362 * t4501 * t21414;
    let t21480 = 8.0_f64 / 15.0_f64 * t4488 * t4494 * t20733;
    let t21483 = 8.0_f64 / 5.0_f64 * t4488 * t12387 * t20737;
    (t21468, t21471, t21474, t21477, t21480, t21483)
}
