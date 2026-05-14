//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 241/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk241<F: Float>(t159: F, t285: F, t695: F, t147: F, t299: F, t169: F, t242: F, t171: F, t465: F, t289: F, t632: F, t274: F, t462: F, t22: F, t461: F) -> (F, F, F, F, F, F, F) {
    let t698 = 0.0002905674151788692 * t695 * t159 * t285;
    let t699 = t299 * t147;
    let t702 = 0.053059442957798957 * t169 * t699 * t242;
    let t703 = t171 * t465;
    let t709 = 0.031835665774679375 * t169 * t289 * t632;
    let t711 = 0.10665013548435875 * t462 * t274;
    let t717 = 1.0 / t22 / t461;
    (t698, t699, t702, t703, t709, t711, t717)
}
