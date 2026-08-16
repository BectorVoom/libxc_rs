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
    let t12296 = piecewise3::<F>(t13, F::cast_from(0.0_f64), -F::cast_from(12.0_f64) * t1083 * t1 * t395 + F::cast_from(24.0_f64) * t12 * t247 + F::cast_from(36.0_f64) * t5422 * t247 + F::cast_from(2.0_f64) * t2938 * t764 - t12294 - F::cast_from(12.0_f64) * t4382);
    let t12300 = t12296 * t44 * t131 * t178 / F::cast_from(30.0_f64);
    t12300
}
