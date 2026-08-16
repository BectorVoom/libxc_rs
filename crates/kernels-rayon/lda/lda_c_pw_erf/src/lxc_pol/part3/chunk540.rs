//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 540/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk540(t411: f64, t440: f64, t2765: f64, t1089: f64, t169: f64, t301: f64, t717: f64, t131: f64, t1552: f64, t137: f64, t142: f64, t1191: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2766 = t440 * t411;
    let t2767 = t2765 * t2766;
    let t2772 = t169 * t717 * t1089 * t301;
    let t2775 = 1.0_f64 / t1552 / t131;
    let t2776 = t2775 * t137;
    let t2777 = t440 * t440;
    let t2778 = t142 * t2777;
    let t2779 = t2776 * t2778;
    let t2782 = t5 * t1191;
    (t2766, t2767, t2772, t2775, t2777, t2778, t2779, t2782)
}
