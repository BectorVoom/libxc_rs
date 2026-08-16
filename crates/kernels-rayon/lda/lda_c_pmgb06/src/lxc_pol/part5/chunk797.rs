//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 797/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk797(t5: f64, t6323: f64, t6325: f64, t4777: f64, t2381: f64, t760: f64, t7290: f64, t44: f64, t131: f64, t155: f64, t2854: f64, t7180: f64, t7445: f64, t7447: f64, t7448: f64, t7449: f64, t7450: f64, t7451: f64, t7452: f64, t7453: f64, t7454: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t7455 = 2.0_f64 / 45.0_f64 * t6323;
    let t7456 = 2.0_f64 / 27.0_f64 * t6325;
    let t7457 = 2.0_f64 / 135.0_f64 * t4777;
    let t7458 = t760 * t2381;
    let t7463 = piecewise3(t6, 0.0_f64, 2.0_f64 * t5 * t7290 + 6.0_f64 * t7458);
    let t7464 = t7463 * t44;
    let t7465 = t7464 * t131;
    let t7467 = t7465 * t155 / 30.0_f64;
    let t7468 = -t7445 + t2854 + 4.0_f64 * t7180 - t7447 - t7448 + t7449 + t7450 + t7451 + t7452 + t7453 + t7454 + t7455 + t7456 - t7457 + t7467;
    (t7455, t7456, t7457, t7458, t7464, t7465, t7467, t7468)
}
