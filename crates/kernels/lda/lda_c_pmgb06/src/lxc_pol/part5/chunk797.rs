//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 797/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk797<F: Float>(t5: F, t6323: F, t6325: F, t4777: F, t2381: F, t760: F, t7290: F, t44: F, t131: F, t155: F, t2854: F, t7180: F, t7445: F, t7447: F, t7448: F, t7449: F, t7450: F, t7451: F, t7452: F, t7453: F, t7454: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t7455 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6323;
    let t7456 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t6325;
    let t7457 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t4777;
    let t7458 = t760 * t2381;
    let t7463 = piecewise3::<F>(t6, F::cast_from(0.0_f64), F::cast_from(2.0_f64) * t5 * t7290 + F::cast_from(6.0_f64) * t7458);
    let t7464 = t7463 * t44;
    let t7465 = t7464 * t131;
    let t7467 = t7465 * t155 / F::cast_from(30.0_f64);
    let t7468 = -t7445 + t2854 + F::cast_from(4.0_f64) * t7180 - t7447 - t7448 + t7449 + t7450 + t7451 + t7452 + t7453 + t7454 + t7455 + t7456 - t7457 + t7467;
    (t7455, t7456, t7457, t7458, t7464, t7465, t7467, t7468)
}
