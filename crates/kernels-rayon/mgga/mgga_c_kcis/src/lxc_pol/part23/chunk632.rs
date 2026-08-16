//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 632/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk632(t187: f64, t1911: f64, t1357: f64, t1585: f64, t1921: f64, t4381: f64, t5540: f64, t5543: f64, t5545: f64, t5548: f64, t5576: f64, t5580: f64, t5587: f64, t5596: f64, t5615: f64, t5620: f64, t6121: f64) -> (f64, f64) {
    let t6125 = t187 * t1911;
    let t6136 = -t5540 + t5543 + t5545 - t5548 + t5576 + t5580 + t187 * t6121 + 0.19751789702565206229e-1_f64 * t187 * t5587 - 0.58482233974552040708e0_f64 * t6125 * t1357 - 0.58482233974552040708e0_f64 * t4381 * t1921 + 0.11696446794910408142e1_f64 * t1585 * t5596 - 0.58482233974552040708e0_f64 * t1585 * t5615 - 0.17315755899375863299e2_f64 * t1585 * t5620;
    (t6125, t6136)
}
