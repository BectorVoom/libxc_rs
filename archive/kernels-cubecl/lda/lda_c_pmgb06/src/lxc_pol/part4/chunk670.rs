//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 670/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk670<F: Float>(t3720: F, t693: F, t681: F, t967: F, t957: F, t963: F, t696: F, t683: F, t978: F, t1179: F, t282: F, t55: F) -> (F, F, F, F, F, F, F) {
    let t3721 = t3720 * t693;
    let t3724 = t967 * t681;
    let t3725 = t963 * t957 * t3724;
    let t3727 = F::cast_from(51.94757731704439_f64) * t696 * t3725;
    let t3729 = t978 * t957 * t683;
    let t3731 = F::cast_from(3.5089341735807875_f64) * t696 * t3729;
    let t3734 = t55 * t1179 * t282;
    (t3721, t3724, t3725, t3727, t3729, t3731, t3734)
}
