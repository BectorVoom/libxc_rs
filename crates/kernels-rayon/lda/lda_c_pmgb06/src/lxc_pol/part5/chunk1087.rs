//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1087/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1087(t5: f64, t1420: f64, t7581: f64, t1426: f64, t439: f64, t7580: f64, t9596: f64, t1072: f64, t19870: f64, t2381: f64, t332: f64, t5961: f64, t7290: f64, t760: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t20086 = t1420 * t7581 / 45.0_f64;
    let t20089 = t439 * t1426 * t7580 / 45.0_f64;
    let t20090 = 4.0_f64 / 405.0_f64 * t9596;
    let t20100 = piecewise3(t6, 0.0_f64, 12.0_f64 * t1072 * t2381 + 2.0_f64 * t19870 * t5 + 2.0_f64 * t332 * t7290 + 6.0_f64 * t5961 * t760);
    (t20086, t20089, t20090, t20100)
}
