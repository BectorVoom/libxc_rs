//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 919/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk919(t1257: f64, t925: f64, t1247: f64, t325: f64, t3537: f64, t3892: f64, t56: f64, t3495: f64, t3527: f64, t1953: f64, t506: f64, t1253: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9828 = t925 * t1257;
    let t9832 = t925 * t1247;
    let t9834 = t325 * t3537;
    let t9836 = t56 * t3892;
    let t9840 = t325 * t3495;
    let t9845 = t325 * t3527;
    let t9847 = t1953 * t506;
    let t9866 = t925 * t1253;
    (t9828, t9832, t9834, t9836, t9840, t9845, t9847, t9866)
}
