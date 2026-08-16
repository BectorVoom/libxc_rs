//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 764/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk764<F: Float>(t5117: F, t5126: F, t5650: F, t5654: F, t5656: F, t5658: F, t6455: F, t6457: F, t6459: F, t6463: F, t6467: F, t6474: F, t6477: F, t6480: F, t6482: F, t6484: F) -> F {
    let t7200 = F::cast_from(0.022363485482220676_f64) * t5650 + t5654 + F::cast_from(0.4328416544945937_f64) * t5656 + F::cast_from(0.1442805514981979_f64) * t5658 - t5117 - t5126 + t6455 - t6457 - t6459 - t6463 - t6467 - t6474 + t6477 + t6480 - t6482 + t6484;
    t7200
}
