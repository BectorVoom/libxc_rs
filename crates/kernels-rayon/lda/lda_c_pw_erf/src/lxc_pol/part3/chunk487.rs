//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 487/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk487(t1960: f64, t221: f64, t325: f64, t790: f64, t1245: f64, t739: f64, t348: f64) -> (f64, f64, f64, f64) {
    let t1962 = 2.0_f64 / 15.0_f64 * t1960 * t221;
    let t1964 = t325 * t790;
    let t1966 = t1245 * t739;
    let t1967 = t1966 * t348;
    (t1962, t1964, t1966, t1967)
}
