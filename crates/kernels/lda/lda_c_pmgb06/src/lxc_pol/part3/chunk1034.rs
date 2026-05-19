//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1034/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1034<F: Float>(t12: F, t1: F, t1083: F, t12294: F, t247: F, t2938: F, t395: F, t4382: F, t5422: F, t764: F, t131: F, t178: F, t44: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t12296 = piecewise3::<F>(t13, F::new(0.0), -F::new(12.0) * t1083 * t1 * t395 + F::new(24.0) * t12 * t247 + F::new(36.0) * t5422 * t247 + F::new(2.0) * t2938 * t764 - t12294 - F::new(12.0) * t4382);
    let t12300 = t12296 * t44 * t131 * t178 / F::new(30.0);
    t12300
}
