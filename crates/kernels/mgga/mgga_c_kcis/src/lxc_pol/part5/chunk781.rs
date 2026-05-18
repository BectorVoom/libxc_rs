//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 781/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk781<F: Float>(t187: F, t1911: F, t1357: F, t1585: F, t1921: F, t4381: F, t5540: F, t5543: F, t5545: F, t5548: F, t5576: F, t5580: F, t5587: F, t5596: F, t5615: F, t5620: F, t6121: F) -> (F, F) {
    let t6125 = t187 * t1911;
    let t6136 = -t5540 + t5543 + t5545 - t5548 + t5576 + t5580 + t187 * t6121 + F::new(0.19751789702565206229e-1) * t187 * t5587 - F::new(0.58482233974552040708e0) * t6125 * t1357 - F::new(0.58482233974552040708e0) * t4381 * t1921 + F::new(0.11696446794910408142e1) * t1585 * t5596 - F::new(0.58482233974552040708e0) * t1585 * t5615 - F::new(0.17315755899375863299e2) * t1585 * t5620;
    (t6125, t6136)
}
