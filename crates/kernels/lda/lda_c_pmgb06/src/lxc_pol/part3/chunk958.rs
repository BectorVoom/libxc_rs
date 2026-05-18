//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 958/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk958<F: Float>(t12: F, t2203: F, t642: F, t1: F, t11039: F, t11047: F, t11050: F, t1219: F, t2200: F, t247: F, t2912: F, t2938: F, t3139: F, t336: F, t337: F, t3548: F, t395: F, t4378: F, t4381: F, t764: F, t8139: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t11282 = F::new(16.0) * t2203 * t642;
    let t11284 = piecewise3::<f64>(t13, F::new(0.0), -F::new(56.0) / F::new(81.0) * t8139 * t764 * t2912 - F::new(16.0) / F::new(9.0) * t3548 * t1 * t11039 + F::new(8.0) / F::new(9.0) * t4378 * t3139 + F::new(4.0) / F::new(3.0) * t1219 * t395 * t337 - F::new(4.0) * t4381 * t11047 + F::new(4.0) / F::new(3.0) * t4381 * t11050 - F::new(2.0) / F::new(9.0) * t2200 * t2938 + F::new(8.0) * t336 * t247 - t11282);
    t11284
}
