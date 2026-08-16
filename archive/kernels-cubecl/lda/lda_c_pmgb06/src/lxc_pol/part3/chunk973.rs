//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 973/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk973<F: Float>(t11381: F, t69: F, t11404: F, t11392: F, t11308: F, t11311: F, t11318: F, t11336: F, t11344: F, t11377: F, t11380: F, t11385: F, t11408: F, t2247: F, t8455: F) -> F {
    let t11515 = t69 * t11381;
    let t11519 = t69 * t11404;
    let t11521 = t69 * t11392;
    let t11524 = -F::cast_from(62.07318_f64) * t2247 * t11408 - t11308 - t11311 + t11318 + t11336 - t11344 + F::cast_from(1.724255_f64) * t11515 - F::cast_from(1.724255_f64) * t69 * t11385 - F::cast_from(2.2990066666666666_f64) * t11519 + F::cast_from(1.7881162962962962_f64) * t11521 - F::cast_from(5.172765_f64) * t8455 + t11377 + t11380;
    t11524
}
