//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1182/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1182<F: Float>(t12: F, t2389: F, t395: F, t1072: F, t1219: F, t19395: F, t2200: F, t336: F, t337: F, t4378: F, t4381: F, t5966: F, t5974: F, t6681: F, t7295: F, t7300: F, t8139: F, zeta_threshold: F) -> (F, F) {
    let t13 = t12 <= zeta_threshold;
    let t21345 = t395 * t2389;
    let t21356 = piecewise3::<F>(t13, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t8139 * t7295 * t337 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t5966 * t1072 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4378 * t6681 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4381 * t21345 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2200 * t5974 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1219 * t7300 * t337 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t336 * t19395);
    (t21345, t21356)
}
