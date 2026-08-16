//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1200/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1200(t5: f64, t1068: f64, t1072: f64, t19870: f64, t2125: f64, t21326: f64, t332: f64, t4486: f64, t4489: f64, t5961: f64, t6042: f64, t6698: f64, t7284: f64, t7290: f64, t8485: f64, t9: f64, zeta_threshold: f64) -> f64 {
    let t6 = t5 <= zeta_threshold;
    let t21750 = piecewise3(t6, 0.0_f64, 40.0_f64 / 81.0_f64 * t8485 * t7284 * t332 - 16.0_f64 / 9.0_f64 * t6042 * t1072 - 8.0_f64 / 9.0_f64 * t4486 * t6698 + 8.0_f64 / 3.0_f64 * t4489 * t21326 + 4.0_f64 / 3.0_f64 * t2125 * t5961 + 4.0_f64 / 9.0_f64 * t1068 * t7290 * t332 + 4.0_f64 / 3.0_f64 * t9 * t19870);
    t21750
}
