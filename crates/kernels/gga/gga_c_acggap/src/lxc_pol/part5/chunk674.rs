//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 674/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk674<F: Float>(t1017: F, t336: F, t4643: F, t1140: F, t1511: F, t1137: F, t1494: F, t1498: F, t1150: F, t335: F, t3383: F, t3385: F, t3394: F, t3410: F, t3412: F, t3428: F, t3432: F, t3446: F, t3449: F, t3454: F, t367: F, t4627: F, t4629: F, t4632: F, t4635: F, t4637: F, t4640: F) -> (F, F, F, F, F) {
    let t4645 = t336 * t4643 * t1017;
    let t4649 = F::new(7.0) / F::new(144.0) * t1140 * t1511;
    let t4651 = F::new(7.0) / F::new(72.0) * t1137 * t1494;
    let t4653 = F::new(7.0) / F::new(72.0) * t1137 * t1498;
    let t4656 = -F::new(0.85748036236139473944e-3) * t3383 + F::new(0.85748036236139473944e-3) * t3385 + F::new(0.17149607247227894789e-2) * t3394 + F::new(0.40015750243531754508e-2) * t3410 - F::new(0.40015750243531754508e-2) * t3412 + F::new(0.42874018118069736972e-3) * t3428 - F::new(0.80031500487063509016e-2) * t3432 + F::new(0.21437009059034868486e-3) * t3446 - t4627 + t4629 - t335 * t4632 / F::new(24.0) - F::new(35.0) / F::new(216.0) * t4635 - F::new(35.0) / F::new(432.0) * t4637 + t1150 * t4640 / F::new(16.0) + t367 * t4645 / F::new(48.0) + t4649 + t4651 + t4653 + F::new(0.10003937560882938627e-2) * t3449 - F::new(0.85748036236139473944e-3) * t3454;
    (t4645, t4649, t4651, t4653, t4656)
}
