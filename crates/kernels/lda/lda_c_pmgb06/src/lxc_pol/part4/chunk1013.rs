//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1013/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1013<F: Float>(t1179: F, t4068: F, t573: F, t580: F, t1147: F, t206: F, t208: F, t31: F, t99: F, t213: F, t398: F, t4075: F) -> (F, F, F, F) {
    let t9457 = t573 * t1179 * t4068;
    let t9461 = F::new(0.006061752703703704) * t580 * t1179 * t4068;
    let t9467 = F::new(0.0002763148940771605) * t206 * t1147 * t99 * t31 * t208;
    let t9470 = t398 * t4075 * t208 * t213;
    (t9457, t9461, t9467, t9470)
}
