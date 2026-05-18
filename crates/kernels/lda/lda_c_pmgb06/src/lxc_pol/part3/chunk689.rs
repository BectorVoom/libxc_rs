//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 689/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk689<F: Float>(t325: F, t326: F, t327: F, t312: F, t754: F, t927: F, t97: F, t374: F, t769: F, t4232: F, t1233: F, t4230: F) -> (F, F, F, F, F, F) {
    let t4343 = F::new(1.0) / t327 / t326 / t325;
    let t4344 = t4343 * t312;
    let t4351 = t927 * t754 * t97;
    let t4354 = t769 * t374;
    let t4355 = t4232 * t4354;
    let t4358 = t1233 * t4230;
    (t4343, t4344, t4351, t4354, t4355, t4358)
}
