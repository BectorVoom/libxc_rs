//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1088/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1088<F: Float>(t12: F, t1952: F, t642: F, t11039: F, t11047: F, t11050: F, t15: F, t1949: F, t2200: F, t2203: F, t247: F, t2912: F, t2938: F, t3139: F, t337: F, t395: F, t4700: F, t598: F, t765: F, zeta_threshold: F) -> F {
    let t13 = t12 <= zeta_threshold;
    let t12960 = F::new(64.0) * t1952 * t642;
    let t12962 = piecewise3::<F>(t13, F::new(0.0), -F::new(80.0) / F::new(81.0) * t2200 * t2912 - F::new(160.0) / F::new(9.0) * t2203 * t11039 + F::new(80.0) / F::new(9.0) * t765 * t3139 - F::new(80.0) / F::new(3.0) * t15 * t395 * t337 + F::new(80.0) * t4700 * t11047 - F::new(80.0) / F::new(3.0) * t4700 * t11050 + F::new(40.0) / F::new(9.0) * t1949 * t2938 + F::new(32.0) * t598 * t247 - t12960);
    t12962
}
