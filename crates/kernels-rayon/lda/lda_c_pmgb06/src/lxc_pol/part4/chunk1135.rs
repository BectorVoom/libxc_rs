//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1135/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1135(t5: f64, t5980: f64, t73: f64, t1068: f64, t1069: f64, t1074: f64, t1100: f64, t11032: f64, t2125: f64, t2377: f64, t2381: f64, t2799: f64, t332: f64, t3912: f64, t395: f64, t4486: f64, t4745: f64, t5961: f64, t6042: f64, t6047: f64, t79: f64, t8485: f64, zeta_threshold: f64) -> (f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t14875 = t73 * t5980;
    let t14909 = piecewise3(t6, 0.0_f64, 40.0_f64 / 81.0_f64 * t8485 * t2377 * t1069 - 64.0_f64 / 27.0_f64 * t4486 * t4745 - 8.0_f64 / 27.0_f64 * t6042 * t1074 + 32.0_f64 / 9.0_f64 * t1068 * t79 * t1100 + 16.0_f64 / 9.0_f64 * t2125 * t395 - 16.0_f64 / 3.0_f64 * t2125 * t2799 - 8.0_f64 / 27.0_f64 * t3912 * t2381 * t1069 + 8.0_f64 / 9.0_f64 * t1068 * t5961 * t332 + 4.0_f64 / 9.0_f64 * t6047 * t1074 + t11032);
    (t14875, t14909)
}
