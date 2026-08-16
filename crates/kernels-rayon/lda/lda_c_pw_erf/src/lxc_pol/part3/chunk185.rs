//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 185/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk185(t493: f64, t496: f64, t155: f64, t56: f64, t174: f64, t177: f64, t188: f64) -> (f64, f64, f64, f64, f64) {
    let t498 = 4.0_f64 / 15.0_f64 * t493 * t496;
    let t499 = t155 * t56;
    let t501 = t174 * t499 * t177;
    let t502 = 0.0018891666666666666_f64 * t501;
    let t503 = t56 * t188;
    (t498, t499, t501, t502, t503)
}
