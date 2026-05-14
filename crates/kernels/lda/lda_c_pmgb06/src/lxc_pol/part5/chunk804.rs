//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 804/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk804<F: Float>(t4103: F, t581: F, t1179: F, t4068: F, t573: F, t580: F, t1147: F, t206: F, t208: F, t31: F, t99: F, t213: F, t398: F, t4075: F, t4641: F, t4913: F, t83: F) -> (F, F, F, F, F, F) {
    let t9429 = 32.0 / 81.0 * t581 * t4103;
    let t9457 = t573 * t1179 * t4068;
    let t9461 = 0.006061752703703704 * t580 * t1179 * t4068;
    let t9467 = 0.0002763148940771605 * t206 * t1147 * t99 * t31 * t208;
    let t9470 = t398 * t4075 * t208 * t213;
    let t9478 = t83 * (-0.33530864197530863 * t4641 + 1.8360493827160493 * t4913) * t208 * t213 / 3.0;
    (t9429, t9457, t9461, t9467, t9470, t9478)
}
