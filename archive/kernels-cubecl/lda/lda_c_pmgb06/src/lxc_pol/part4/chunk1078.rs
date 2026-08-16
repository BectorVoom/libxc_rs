//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1078/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1078<F: Float>(t3073: F, t831: F, t132: F, t435: F, t4681: F, t1842: F, t642: F, t5375: F, t591: F, t4111: F, t5378: F, t5382: F) -> (F, F, F, F, F, F) {
    let t12278 = t831 * t3073;
    let t12281 = t132 * t435 * t4681;
    let t12294 = F::cast_from(48.0_f64) * t1842 * t642;
    let t12304 = t5375 * t591;
    let t12306 = t5378 * t4111;
    let t12307 = F::cast_from(2e-21_f64) * t12306;
    let t12308 = t5382 * t591;
    (t12278, t12281, t12294, t12304, t12307, t12308)
}
