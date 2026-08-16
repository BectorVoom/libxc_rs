//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1181/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1181(t5: f64, t11228: f64, t2381: f64, t395: f64, t1072: f64, t1212: f64, t19870: f64, t2192: f64, t330: f64, t332: f64, t4363: f64, t4366: f64, t5953: f64, t5961: f64, t6698: f64, t7284: f64, t7290: f64, t8119: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t21317 = 5.84605_f64 * t11228;
    let t21326 = t395 * t2381;
    let t21337 = piecewise3(t6, 0.0_f64, -56.0_f64 / 81.0_f64 * t8119 * t7284 * t332 + 16.0_f64 / 9.0_f64 * t5953 * t1072 + 8.0_f64 / 9.0_f64 * t4363 * t6698 - 4.0_f64 / 3.0_f64 * t4366 * t21326 - 2.0_f64 / 3.0_f64 * t2192 * t5961 - 2.0_f64 / 9.0_f64 * t1212 * t7290 * t332 + 2.0_f64 / 3.0_f64 * t330 * t19870);
    (t21317, t21326, t21337)
}
