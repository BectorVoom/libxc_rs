//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1395/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1395<F: Float>(t1585: F, t17942: F, t187: F, t1921: F, t21311: F, t21316: F, t21327: F, t21337: F, t21351: F, t21353: F, t21356: F, t21359: F, t21362: F, t21365: F, t21369: F, t21372: F, t21376: F, t21402: F, t4381: F, t5596: F, t5620: F, t6125: F, t7021: F) -> F {
    let t23023 = F::new(0.23392893589820816284e1) * t1585 * t21327 - F::new(0.1025389702100779493e4) * t1585 * t21316 + F::new(0.23392893589820816284e1) * t6125 * t5596 - F::new(0.11696446794910408142e1) * t17942 * t1921 - t21351 + t21353 + t21356 - t21359 - t21362 - t21365 + t21369 + t21372 + t21376 - F::new(0.58482233974552040708e0) * t4381 * t7021 - t21402 + F::new(0.19751789702565206229e-1) * t187 * t21311 - F::new(0.346315117987517266e2) * t6125 * t5620 - F::new(0.35089340384731224426e1) * t1585 * t21337;
    t23023
}
