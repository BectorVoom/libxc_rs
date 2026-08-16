//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 900/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk900(t1549: f64, t2810: f64, t169: f64, t301: f64, t3196: f64, t717: f64, t142: f64, t3251: f64, t2775: f64, t450: f64, t2778: f64, t147: f64, t159: f64, t285: f64, t3165: f64) -> (f64, f64, f64, f64, f64) {
    let t9141 = t1549 * t2810;
    let t9146 = t169 * t717 * t3196 * t301;
    let t9148 = t142 * t3251;
    let t9156 = t2775 * t450;
    let t9157 = t9156 * t2778;
    let t9163 = 1.0943113336969376e-06_f64 * t3165 * t147 * t159 * t285;
    (t9141, t9146, t9148, t9157, t9163)
}
