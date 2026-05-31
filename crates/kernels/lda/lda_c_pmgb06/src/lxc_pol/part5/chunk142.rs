//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 142/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk142<F: Float>(t349: F, t350: F, t342: F, t38: F, t56: F, t109: F, t54: F, t55: F, t30: F, t53: F) -> (F, F, F, F) {
    let t352 = F::cast_from(0.48717083333333333_f64) * t349 * t350;
    let t355 = F::cast_from(2.923025_f64) * t38 * t56 * t342;
    let t359 = t54 * t55 * t109 * t56 / F::cast_from(12.0_f64);
    let t360 = t53 * t30;
    (t352, t355, t359, t360)
}
