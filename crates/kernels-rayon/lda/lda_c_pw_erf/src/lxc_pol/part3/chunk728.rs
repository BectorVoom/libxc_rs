//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 728/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk728(t4592: f64, t2123: f64, t565: f64, t790: f64, t925: f64, t1968: f64, t325: f64, t1973: f64, t2869: f64, t4: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4593 = 4.0_f64 / 135.0_f64 * t4592;
    let t4595 = 8.0_f64 / 45.0_f64 * t565 * t2123;
    let t4600 = t925 * t790;
    let t4602 = t325 * t1968;
    let t4604 = t325 * t1973;
    let t4605 = 0.002518888888888889_f64 * t4604;
    let t4606 = t4 * t2869;
    (t4593, t4595, t4600, t4602, t4604, t4605, t4606)
}
