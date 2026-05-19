//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 990/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk990<F: Float>(t1018: F, t138: F, t3698: F, t1023: F, t1026: F, t409: F, t109: F, t3674: F, t3676: F, t1767: F, t282: F, t55: F, t691: F) -> (F, F, F, F) {
    let t8647 = F::cast_from(0.07123333333333333_f64) * t138 * t1018 * t3698;
    let t8651 = F::cast_from(0.2849333333333333_f64) * t138 * t409 * t1023 * t1026;
    let t8655 = F::cast_from(6.87343803774119_f64) * t138 * t109 * t3674 * t3676;
    let t8659 = F::cast_from(0.0018989649058080863_f64) * t691 * t55 * t1767 * t282;
    (t8647, t8651, t8655, t8659)
}
