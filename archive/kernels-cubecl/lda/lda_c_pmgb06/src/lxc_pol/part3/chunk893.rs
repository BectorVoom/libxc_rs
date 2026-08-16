//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 893/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk893<F: Float>(t1392: F, t1512: F, t3068: F, t432: F, t161: F, t2886: F, t489: F, t1179: F, t4068: F, t573: F, t580: F, t1147: F, t206: F, t208: F, t31: F, t99: F) -> (F, F, F, F, F, F) {
    let t9441 = t1512 * t1392;
    let t9443 = t432 * t3068;
    let t9450 = t161 * t489 * t2886;
    let t9457 = t573 * t1179 * t4068;
    let t9461 = F::cast_from(0.006061752703703704_f64) * t580 * t1179 * t4068;
    let t9467 = F::cast_from(0.0002763148940771605_f64) * t206 * t1147 * t99 * t31 * t208;
    (t9441, t9443, t9450, t9457, t9461, t9467)
}
