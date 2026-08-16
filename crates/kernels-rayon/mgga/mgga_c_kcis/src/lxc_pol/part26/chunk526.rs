//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 526/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk526(t1347: f64, t1356: f64, t5613: f64, t1919: f64, t3944: f64, t1354: f64, t3947: f64, t1345: f64, t1357: f64, t1921: f64, t3914: f64, t45: f64, t5540: f64, t5543: f64, t5545: f64, t5548: f64, t5576: f64, t5580: f64, t5587: f64, t5590: f64, t5596: f64) -> (f64, f64, f64, f64, f64) {
    let t5615 = t1347 * t5613 * t1356;
    let t5618 = t3944 * t1919;
    let t5619 = t3947 * t1354;
    let t5620 = t5618 * t5619;
    let t5623 = -t5540 + t5543 + t5545 - t5548 + t5576 + t5580 + 0.19751789702565206229e-1_f64 * t45 * t5587 - 0.58482233974552040708e0_f64 * t5590 * t1357 - 0.58482233974552040708e0_f64 * t3914 * t1921 + 0.11696446794910408142e1_f64 * t1345 * t5596 - 0.58482233974552040708e0_f64 * t1345 * t5615 - 0.17315755899375863299e2_f64 * t1345 * t5620;
    (t5615, t5618, t5619, t5620, t5623)
}
