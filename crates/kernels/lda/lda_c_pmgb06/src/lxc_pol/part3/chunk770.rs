//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 770/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk770<F: Float>(t1767: F, t55: F, t32: F, t4238: F, t107: F, t4913: F, t642: F, t93: F, t297: F, t301: F, t83: F, t1193: F, t398: F, t4001: F, t4299: F, t2841: F, t4297: F) -> (F, F, F, F, F, F) {
    let t8165 = t55 * t1767;
    let t8170 = t32 * t4238;
    let t8173 = -70.0 / 81.0 * t93 * t8165 + 0.22252592592592593 * t4913 - 0.07316671043820612 * t642 + 0.015663796296296297 * t107 * t8170;
    let t8177 = 0.01197423401025461 * t297 * t83 * t8173 * t301;
    let t8180 = t4001 * t398 * t1193 * t4299;
    let t8184 = 1.8276876377896586e-05 * t4297 * t2841 * t4299;
    (t8165, t8170, t8173, t8177, t8180, t8184)
}
