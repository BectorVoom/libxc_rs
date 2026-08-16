//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1296/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1296(t1347: f64, t1356: f64, t21447: f64, t1345: f64, t16093: f64, t1921: f64, t21351: f64, t21353: f64, t21356: f64, t21359: f64, t21362: f64, t21365: f64, t21369: f64, t21372: f64, t21376: f64, t21402: f64, t3914: f64, t5590: f64, t5596: f64, t5620: f64, t7004: f64, t7021: f64, t7025: f64) -> (f64, f64) {
    let t21449 = t1347 * t21447 * t1356;
    let t21452 = -0.11696446794910408142e1_f64 * t16093 * t1921 - 0.346315117987517266e2_f64 * t5590 * t5620 - t21351 + t21353 + t21356 - t21359 - t21362 - t21365 + t21369 + t21372 + t21376 + 0.23392893589820816284e1_f64 * t5590 * t5596 - 0.17315755899375863299e2_f64 * t3914 * t7025 - 0.58482233974552040708e0_f64 * t3914 * t7021 + 0.11696446794910408142e1_f64 * t3914 * t7004 - t21402 - 0.58482233974552040708e0_f64 * t1345 * t21449;
    (t21449, t21452)
}
