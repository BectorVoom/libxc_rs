//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1033/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1033<F: Float>(t12: F, t2389: F, t395: F, t1072: F, t1219: F, t19395: F, t2200: F, t336: F, t337: F, t4378: F, t4381: F, t5966: F, t5974: F, t6681: F, t7295: F, t7300: F, t8139: F, zeta_threshold: F) -> (F, F) {
    let t13 = t12 <= zeta_threshold;
    let t21345 = t395 * t2389;
    let t21356 = piecewise3(t13, 0.0, -56.0 / 81.0 * t8139 * t7295 * t337 - 16.0 / 9.0 * t5966 * t1072 + 8.0 / 9.0 * t4378 * t6681 + 4.0 / 3.0 * t4381 * t21345 - 2.0 / 3.0 * t2200 * t5974 - 2.0 / 9.0 * t1219 * t7300 * t337 + 2.0 / 3.0 * t336 * t19395);
    (t21345, t21356)
}
