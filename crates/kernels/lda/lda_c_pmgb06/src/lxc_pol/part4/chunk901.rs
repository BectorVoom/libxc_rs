//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 901/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk901<F: Float>(t1040: F, t1044: F, t138: F, t409: F, t1018: F, t3698: F, t1023: F, t1026: F, t109: F, t3674: F, t3676: F, t1767: F, t282: F, t55: F, t691: F, t1112: F, t3720: F) -> (F, F, F, F, F, F) {
    let t8644 = 2.2911460125803966 * t138 * t409 * t1040 * t1044;
    let t8647 = 0.07123333333333333 * t138 * t1018 * t3698;
    let t8651 = 0.2849333333333333 * t138 * t409 * t1023 * t1026;
    let t8655 = 6.87343803774119 * t138 * t109 * t3674 * t3676;
    let t8659 = 0.0018989649058080863 * t691 * t55 * t1767 * t282;
    let t8663 = t3720 * t1112;
    (t8644, t8647, t8651, t8655, t8659, t8663)
}
