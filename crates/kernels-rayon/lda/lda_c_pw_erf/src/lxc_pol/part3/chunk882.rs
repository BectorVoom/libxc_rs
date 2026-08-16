//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 882/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk882(t40: f64, t60: f64, t8600: f64, t8639: f64, t8686: f64, t8729: f64, t174: f64, t3046: f64, t3105: f64, t3027: f64, t3112: f64, t169: f64, t2817: f64, t301: f64, t678: f64) -> (f64, f64, f64, f64) {
    let t8733 = t40 * t60 * (t8600 + t8639 + t8686 + t8729);
    let t8734 = t60 * t174;
    let t8737 = 0.1301229705933783_f64 * t8734 * t3046 * t3105;
    let t8740 = 1.9263778438055648_f64 * t8734 * t3027 * t3112;
    let t8751 = t169 * t2817 * t678 * t301;
    (t8733, t8737, t8740, t8751)
}
