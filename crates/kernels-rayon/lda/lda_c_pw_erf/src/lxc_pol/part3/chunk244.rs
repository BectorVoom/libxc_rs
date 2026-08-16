//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 244/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk244(t159: f64, t285: f64, t695: f64, t147: f64, t299: f64, t169: f64, t242: f64, t171: f64, t465: f64, t289: f64, t632: f64, t274: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t698 = 0.0002905674151788692_f64 * t695 * t159 * t285;
    let t699 = t299 * t147;
    let t702 = 0.053059442957798957_f64 * t169 * t699 * t242;
    let t703 = t171 * t465;
    let t709 = 0.031835665774679375_f64 * t169 * t289 * t632;
    let t711 = 0.10665013548435875_f64 * t462 * t274;
    (t698, t699, t702, t703, t709, t711)
}
