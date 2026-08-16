//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 916/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk916<F: Float>(t1347: F, t1356: F, t21447: F, t1345: F, t16093: F, t1921: F, t21351: F, t21353: F, t21356: F, t21359: F, t21362: F, t21365: F, t21369: F, t21372: F, t21376: F, t21402: F, t3914: F, t5590: F, t5596: F, t5620: F, t7004: F, t7021: F, t7025: F) -> (F, F) {
    let t21449 = t1347 * t21447 * t1356;
    let t21452 = -F::cast_from(0.11696446794910408142e1_f64) * t16093 * t1921 - F::cast_from(0.346315117987517266e2_f64) * t5590 * t5620 - t21351 + t21353 + t21356 - t21359 - t21362 - t21365 + t21369 + t21372 + t21376 + F::cast_from(0.23392893589820816284e1_f64) * t5590 * t5596 - F::cast_from(0.17315755899375863299e2_f64) * t3914 * t7025 - F::cast_from(0.58482233974552040708e0_f64) * t3914 * t7021 + F::cast_from(0.11696446794910408142e1_f64) * t3914 * t7004 - t21402 - F::cast_from(0.58482233974552040708e0_f64) * t1345 * t21449;
    (t21449, t21452)
}
