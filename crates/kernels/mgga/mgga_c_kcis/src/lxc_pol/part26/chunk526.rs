//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 526/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk526<F: Float>(t1347: F, t1356: F, t5613: F, t1919: F, t3944: F, t1354: F, t3947: F, t1345: F, t1357: F, t1921: F, t3914: F, t45: F, t5540: F, t5543: F, t5545: F, t5548: F, t5576: F, t5580: F, t5587: F, t5590: F, t5596: F) -> (F, F, F, F, F) {
    let t5615 = t1347 * t5613 * t1356;
    let t5618 = t3944 * t1919;
    let t5619 = t3947 * t1354;
    let t5620 = t5618 * t5619;
    let t5623 = -t5540 + t5543 + t5545 - t5548 + t5576 + t5580 + F::new(0.19751789702565206229e-1) * t45 * t5587 - F::new(0.58482233974552040708e0) * t5590 * t1357 - F::new(0.58482233974552040708e0) * t3914 * t1921 + F::new(0.11696446794910408142e1) * t1345 * t5596 - F::new(0.58482233974552040708e0) * t1345 * t5615 - F::new(0.17315755899375863299e2) * t1345 * t5620;
    (t5615, t5618, t5619, t5620, t5623)
}
