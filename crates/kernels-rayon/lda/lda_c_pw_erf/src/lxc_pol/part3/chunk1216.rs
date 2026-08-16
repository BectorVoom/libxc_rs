//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1216/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1216(t1325: f64, t3787: f64, t4881: f64, t5393: f64, t519: f64, t5359: f64, t1519: f64, t1982: f64, t10722: f64, t10725: f64, t10729: f64, t2072: f64, t4073: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14343 = t1325 * t3787 * t4881;
    let t14344 = 16.0_f64 / 15.0_f64 * t14343;
    let t14346 = t1325 * t3787 * t5393;
    let t14347 = 8.0_f64 / 15.0_f64 * t14346;
    let t14349 = t519 * t3787 * t5359;
    let t14350 = 8.0_f64 / 15.0_f64 * t14349;
    let t14351 = t1982 * t1519;
    let t14352 = 4.0_f64 / 45.0_f64 * t14351;
    let t14353 = 8.0_f64 / 15.0_f64 * t10722;
    let t14354 = 8.0_f64 / 15.0_f64 * t10725;
    let t14355 = 16.0_f64 / 45.0_f64 * t10729;
    let t14357 = 4.0_f64 / 5.0_f64 * t4073 * t2072;
    (t14344, t14347, t14350, t14352, t14353, t14354, t14355, t14357)
}
