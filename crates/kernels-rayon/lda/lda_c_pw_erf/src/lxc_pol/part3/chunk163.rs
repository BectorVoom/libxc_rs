//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 163/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk163(t128: f64, t431: f64, t325: f64, t96: f64) -> (f64, f64, f64, f64) {
    let t432 = t431 * t128;
    let t434 = 0.24484_f64 * t432 * t325;
    let t435 = t96 * t96;
    let t436 = 1.0_f64 / t435;
    (t432, t434, t435, t436)
}
