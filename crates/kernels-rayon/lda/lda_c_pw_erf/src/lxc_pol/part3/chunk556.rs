//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 556/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk556(t169: f64, t289: f64, t2929: f64, t274: f64, t343: f64, t39: f64, t678: f64, t1089: f64, t462: f64, t2700: f64, t2703: f64, t2706: f64, t2709: f64, t2712: f64, t2739: f64, t2741: f64, t2744: f64, t2748: f64, t2752: f64, t2755: f64, t2759: f64) -> (f64, f64, f64, f64, f64) {
    let t2932 = 0.031835665774679375_f64 * t169 * t289 * t2929;
    let t2934 = 1.279801625812305_f64 * t343 * t274;
    let t2935 = t39 * t678;
    let t2937 = t462 * t1089;
    let t2939 = t2700 + t2703 + t2706 - t2709 - t2712 - t2739 - t2741 - t2744 - t2748 + t2752 - t2755 + t2759;
    (t2932, t2934, t2935, t2937, t2939)
}
