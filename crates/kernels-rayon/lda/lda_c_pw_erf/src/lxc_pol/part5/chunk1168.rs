//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1168/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1168(t21348: f64, t15563: f64, t799: f64, t2178: f64, t6988: f64, t15694: f64, t2558: f64, t3883: f64, t519: f64, t7484: f64, t2171: f64, t6682: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21349 = 16.0_f64 / 45.0_f64 * t21348;
    let t21351 = 8.0_f64 / 15.0_f64 * t15563 * t799;
    let t21353 = 16.0_f64 / 15.0_f64 * t6988 * t2178;
    let t21355 = 8.0_f64 / 5.0_f64 * t15694 * t2558;
    let t21357 = t519 * t3883 * t7484;
    let t21358 = 16.0_f64 / 27.0_f64 * t21357;
    let t21359 = t2171 * t6682;
    (t21349, t21351, t21353, t21355, t21358, t21359)
}
