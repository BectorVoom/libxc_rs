//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 142/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk142<F: Float>(t349: F, t350: F, t342: F, t38: F, t56: F, t109: F, t54: F, t55: F, t30: F, t53: F) -> (F, F, F, F) {
    let t352 = 0.48717083333333333 * t349 * t350;
    let t355 = 2.923025 * t38 * t56 * t342;
    let t359 = t54 * t55 * t109 * t56 / 12.0;
    let t360 = t53 * t30;
    (t352, t355, t359, t360)
}
