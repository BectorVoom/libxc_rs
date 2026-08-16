//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1038/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1038(t12158: f64, t12160: f64, t571: f64, t1472: f64, t4873: f64, t1278: f64, t1976: f64, t4848: f64, t519: f64, t1987: f64, t3709: f64, t1446: f64, t4856: f64) -> (f64, f64, f64, f64, f64) {
    let t12163 = 64.0_f64 / 27.0_f64 * t571 * t12158 * t12160;
    let t12165 = 4.0_f64 / 15.0_f64 * t1472 * t4873;
    let t12169 = 8.0_f64 / 15.0_f64 * t519 * t4848 * t1976 * t1278;
    let t12171 = 8.0_f64 / 15.0_f64 * t3709 * t1987;
    let t12173 = 8.0_f64 / 15.0_f64 * t1446 * t4856;
    (t12163, t12165, t12169, t12171, t12173)
}
