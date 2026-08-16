//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 182/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk182(t482: f64, t483: f64, t485: f64, t163: f64, t169: f64, t234: f64, t299: f64, t172: f64, t181: f64, t184: f64) -> (f64, f64, f64, f64) {
    let t487 = 0.001975389032890948_f64 * t482 * t483 * t485;
    let t491 = 0.008980675507690957_f64 * t169 * t299 * t234 * t163;
    let t492 = t172 * t181;
    let t493 = t492 * t184;
    (t487, t491, t492, t493)
}
