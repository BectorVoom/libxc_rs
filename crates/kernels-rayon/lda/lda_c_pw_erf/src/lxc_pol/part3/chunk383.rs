//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 383/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk383(t1397: f64, t548: f64, t218: f64, t580: f64) -> (f64, f64, f64) {
    let t1398 = t548 * t1397;
    let t1399 = 16.0_f64 / 45.0_f64 * t1398;
    let t1401 = 1.0_f64 / t580 / t218;
    (t1398, t1399, t1401)
}
