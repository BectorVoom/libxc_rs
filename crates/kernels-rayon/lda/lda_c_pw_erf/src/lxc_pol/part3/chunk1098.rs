//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1098/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1098(t12841: f64, t3430: f64, t4763: f64, t1315: f64, t5327: f64, t2023: f64, t3727: f64, t1308: f64, t352: f64, t5029: f64, t558: f64, t571: f64) -> (f64, f64, f64, f64, f64) {
    let t12842 = 16.0_f64 / 45.0_f64 * t12841;
    let t12844 = 8.0_f64 / 9.0_f64 * t4763 * t3430;
    let t12846 = 8.0_f64 / 15.0_f64 * t5327 * t1315;
    let t12848 = 4.0_f64 / 15.0_f64 * t3727 * t2023;
    let t12853 = 4.0_f64 / 15.0_f64 * t571 * t1308 * t5029 * t558 * t352;
    (t12842, t12844, t12846, t12848, t12853)
}
