//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 767/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk767<F: Float>(t7633: F, t7636: F, t7638: F, t7642: F, t7644: F, t7648: F, t7650: F, t7653: F, t7655: F, t7658: F, t7662: F, t7665: F, t7669: F, t7673: F, t7676: F, t7678: F, t7683: F) -> (F,) {
    let t7949 = t7633 + t7636 + t7638 + t7642 + t7644 - t7648 - t7650 - t7653 - t7655 - t7658 - t7662 + t7665 + t7669 + t7673 + t7676 + t7678 + t7683;
    (t7949,)
}
