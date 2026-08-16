//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1395/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1395(t1585: f64, t17942: f64, t187: f64, t1921: f64, t21311: f64, t21316: f64, t21327: f64, t21337: f64, t21351: f64, t21353: f64, t21356: f64, t21359: f64, t21362: f64, t21365: f64, t21369: f64, t21372: f64, t21376: f64, t21402: f64, t4381: f64, t5596: f64, t5620: f64, t6125: f64, t7021: f64) -> f64 {
    let t23023 = 0.23392893589820816284e1_f64 * t1585 * t21327 - 0.1025389702100779493e4_f64 * t1585 * t21316 + 0.23392893589820816284e1_f64 * t6125 * t5596 - 0.11696446794910408142e1_f64 * t17942 * t1921 - t21351 + t21353 + t21356 - t21359 - t21362 - t21365 + t21369 + t21372 + t21376 - 0.58482233974552040708e0_f64 * t4381 * t7021 - t21402 + 0.19751789702565206229e-1_f64 * t187 * t21311 - 0.346315117987517266e2_f64 * t6125 * t5620 - 0.35089340384731224426e1_f64 * t1585 * t21337;
    t23023
}
