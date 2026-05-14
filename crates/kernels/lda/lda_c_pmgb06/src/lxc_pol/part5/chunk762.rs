//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 762/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk762<F: Float>(t7468: F, t7545: F, t7631: F, t7679: F, t7709: F, t7745: F, t7764: F, t7866: F, t3: F, t7364: F, t117: F, t118: F, t123: F, t125: F, t2793: F, t2797: F, t2812: F, t2820: F, t2825: F, t2840: F, t2844: F, t2846: F, t3481: F, t7157: F) -> (F, F, F) {
    let t7869 = t7468 + t7545 + t7631 + t7679 + t7709 + t7745 + t7764 + t7866;
    let t7874 = t3 * t7364;
    let t7878 = -t2793 + t2797 - t2812 + t2820 + t2825 - t2840 - t2844 - t2846 + t3481 - 0.005388405304614574 * t123 * t125 * t7869 * t117 - 0.031505407223141116 * t7874 * t118 - 0.005926167098672845 * t7157;
    (t7869, t7874, t7878)
}
