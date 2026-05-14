//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1046/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1046<F: Float>(t12655: F, t12656: F, t12664: F, t12665: F, t12666: F, t12672: F, t12676: F, t12678: F, t12682: F, t12686: F, t12690: F, t12696: F, t12700: F, t12704: F, t12708: F, t12710: F, t12713: F, t12719: F, t12724: F, t12726: F, t12728: F, t12730: F, t12733: F) -> (F, F) {
    let t14385 = t12655 + t12656 - t12664 - t12665 - t12666 - t12672 + t12676 - t12678 - t12682 + t12686 + t12690;
    let t14386 = -t12696 - t12700 - t12704 - t12708 - t12710 - t12713 - t12719 - t12724 - t12726 - t12728 - t12730 - t12733;
    (t14385, t14386)
}
