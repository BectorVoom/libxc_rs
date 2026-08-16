//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 271/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk271(t557: f64, t816: f64, t11: f64, t556: f64, t203: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t817 = t557 * t816;
    let t818 = t11 * t817;
    let t820 = t556 + 0.0018891666666666666_f64 * t818;
    let t821 = t203 * t820;
    let t822 = t821 * t184;
    (t817, t818, t820, t821, t822)
}
