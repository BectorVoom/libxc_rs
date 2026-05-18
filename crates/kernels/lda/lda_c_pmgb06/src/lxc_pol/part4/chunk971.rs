//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 971/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk971<F: Float>(t2789: F, t297: F, t301: F, t398: F, t1767: F, t55: F, t32: F, t4238: F, t107: F, t4913: F, t642: F, t93: F) -> (F, F, F, F) {
    let t8163 = t297 * t398 * t2789 * t301;
    let t8165 = t55 * t1767;
    let t8170 = t32 * t4238;
    let t8173 = -F::new(70.0) / F::new(81.0) * t93 * t8165 + F::new(0.22252592592592593) * t4913 - F::new(0.07316671043820612) * t642 + F::new(0.015663796296296297) * t107 * t8170;
    (t8163, t8165, t8170, t8173)
}
