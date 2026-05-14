//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 749/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk749<F: Float>(t496: F, t7670: F, t493: F, t6130: F, t834: F, t6134: F, t835: F, t7633: F, t7636: F, t7638: F, t7642: F, t7644: F, t7648: F, t7650: F, t7653: F, t7655: F, t7658: F, t7662: F, t7665: F, t7669: F) -> (F, F, F, F, F, F) {
    let t7671 = t496 * t7670;
    let t7673 = 2.0 / 15.0 * t493 * t7671;
    let t7674 = t6130 * t834;
    let t7676 = t493 * t7674 / 15.0;
    let t7678 = t6134 * t835 / 15.0;
    let t7679 = t7633 + t7636 + t7638 + t7642 + t7644 - t7648 - t7650 - t7653 - t7655 - t7658 - t7662 + t7665 + t7669 + t7673 + t7676 + t7678;
    (t7671, t7673, t7674, t7676, t7678, t7679)
}
