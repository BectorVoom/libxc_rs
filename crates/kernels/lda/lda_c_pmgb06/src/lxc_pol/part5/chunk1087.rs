//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1087/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1087<F: Float>(t5: F, t1420: F, t7581: F, t1426: F, t439: F, t7580: F, t9596: F, t1072: F, t19870: F, t2381: F, t332: F, t5961: F, t7290: F, t760: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t20086 = t1420 * t7581 / F::cast_from(45.0_f64);
    let t20089 = t439 * t1426 * t7580 / F::cast_from(45.0_f64);
    let t20090 = F::cast_from(4.0_f64) / F::cast_from(405.0_f64) * t9596;
    let t20100 = piecewise3::<F>(t6, F::cast_from(0.0_f64), F::cast_from(12.0_f64) * t1072 * t2381 + F::cast_from(2.0_f64) * t19870 * t5 + F::cast_from(2.0_f64) * t332 * t7290 + F::cast_from(6.0_f64) * t5961 * t760);
    (t20086, t20089, t20090, t20100)
}
