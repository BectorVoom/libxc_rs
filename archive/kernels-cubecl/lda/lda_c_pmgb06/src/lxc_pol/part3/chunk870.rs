//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 870/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk870<F: Float>(t3738: F, t696: F, t8599: F, t967: F, t3705: F, t971: F, t138: F, t258: F, t3834: F, t3862: F, t3818: F, t3859: F) -> (F, F, F, F) {
    let t8737 = F::cast_from(623.3709278045327_f64) * t696 * t3738 * t8599 * t967;
    let t8738 = t971 * t3705;
    let t8740 = t258 * t138;
    let t8743 = F::cast_from(0.13012297560362088_f64) * t8740 * t3834 * t3862;
    let t8746 = F::cast_from(1.9263893255070628_f64) * t8740 * t3818 * t3859;
    (t8737, t8738, t8743, t8746)
}
