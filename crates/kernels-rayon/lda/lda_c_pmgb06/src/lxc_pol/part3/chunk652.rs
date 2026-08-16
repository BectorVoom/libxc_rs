//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 652/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk652(t285: f64, t3933: f64, t248: f64, t3874: f64, t3877: f64, t3881: f64, t3884: f64, t3888: f64, t3899: f64, t3901: f64, t3904: f64, t3906: f64, t3908: f64, t3911: f64) -> (f64, f64) {
    let t3934 = t3933 * t285;
    let t3936 = t248 * t3934 + t3874 + t3877 + t3881 - t3884 - t3888 - 12.0_f64 * t3899 + 24.0_f64 * t3901 + 3.0_f64 * t3904 - 96.0_f64 * t3906 + 60.0_f64 * t3908 + t3911;
    (t3934, t3936)
}
