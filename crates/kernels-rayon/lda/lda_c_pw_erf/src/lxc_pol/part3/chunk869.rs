//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 869/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk869(t174: f64, t3149: f64, t998: f64, t155: f64, t3127: f64, t3131: f64, t3135: f64, t3137: f64, t2745: f64, t3123: f64, t19: f64, t4288: f64, t729: f64, t734: f64) -> (f64, f64, f64, f64, f64) {
    let t8389 = 0.07123333333333333_f64 * t174 * t998 * t3149;
    let t8393 = 36.84545214203136_f64 * t174 * t155 * t3127 * t3131;
    let t8397 = 6.873371715287382_f64 * t174 * t155 * t3135 * t3137;
    let t8400 = 0.4274_f64 * t174 * t2745 * t3123;
    let t8403 = t4288 * t729 * t19 * t734;
    (t8389, t8393, t8397, t8400, t8403)
}
