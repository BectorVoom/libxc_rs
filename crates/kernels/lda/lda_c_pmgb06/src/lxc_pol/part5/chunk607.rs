//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 607/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk607<F: Float>(t4232: F, t4354: F, t1233: F, t4230: F, t387: F, t73: F, t2249: F, t3537: F, t760: F, t1: F, t1212: F, t3548: F, t764: F) -> (F, F, F, F, F, F, F) {
    let t4355 = t4232 * t4354;
    let t4358 = t1233 * t4230;
    let t4359 = t387 * t73;
    let t4360 = t4359 * t2249;
    let t4363 = t3537 * t760;
    let t4366 = t1212 * t1;
    let t4378 = t3548 * t764;
    (t4355, t4358, t4359, t4360, t4363, t4366, t4378)
}
