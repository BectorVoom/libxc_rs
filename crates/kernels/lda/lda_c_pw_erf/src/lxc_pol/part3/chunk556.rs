//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 556/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk556<F: Float>(t169: F, t289: F, t2929: F, t274: F, t343: F, t39: F, t678: F, t1089: F, t462: F, t2700: F, t2703: F, t2706: F, t2709: F, t2712: F, t2739: F, t2741: F, t2744: F, t2748: F, t2752: F, t2755: F, t2759: F) -> (F, F, F, F, F) {
    let t2932 = F::cast_from(0.031835665774679375_f64) * t169 * t289 * t2929;
    let t2934 = F::cast_from(1.279801625812305_f64) * t343 * t274;
    let t2935 = t39 * t678;
    let t2937 = t462 * t1089;
    let t2939 = t2700 + t2703 + t2706 - t2709 - t2712 - t2739 - t2741 - t2744 - t2748 + t2752 - t2755 + t2759;
    (t2932, t2934, t2935, t2937, t2939)
}
