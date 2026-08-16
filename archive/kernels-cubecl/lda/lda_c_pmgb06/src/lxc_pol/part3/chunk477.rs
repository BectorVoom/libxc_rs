//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 477/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk477<F: Float>(t12: F, t1: F, t598: F, t1949: F, t337: F, t395: F, t1948: F, t44: F, t441: F, t813: F, zeta_threshold: F) -> (F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t1952 = t598 * t1;
    let t1956 = piecewise3::<F>(t13, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1949 * t337 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t1952 * t395);
    let t1959 = (t1948 / F::cast_from(2.0_f64) + t1956 / F::cast_from(2.0_f64)) * t44;
    let t1962 = t441 * t813;
    (t1952, t1959, t1962)
}
