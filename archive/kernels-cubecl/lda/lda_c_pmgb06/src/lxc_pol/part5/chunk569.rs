//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 569/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk569<F: Float>(t315: F, t342: F, t934: F, t3566: F, t1238: F, t56: F, t97: F, t409: F, t55: F, t1276: F, t1243: F, t19: F, t369: F) -> (F, F, F, F, F, F) {
    let t3568 = t934 * t315 * t342;
    let t3569 = t3566 * t3568;
    let t3576 = t1238 * t56 * t97;
    let t3577 = t3576 * t3568;
    let t3582 = t55 * t409 * t342;
    let t3583 = t1276 * t3582;
    let t3603 = t1243 * t3582;
    let t3615 = F::cast_from(1.0_f64) / t369 / t19;
    (t3569, t3576, t3577, t3583, t3603, t3615)
}
