//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 820/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk820<F: Float>(t5253: F, t969: F, t1212: F, t1221: F, t1225: F, t1226: F, t1831: F, t1835: F, t3545: F, t3550: F, t3575: F, t3582: F, t3585: F, t3592: F, t405: F, t4684: F, t4687: F, t4689: F, t4692: F, t4721: F, t4725: F, t4732: F, t5208: F, t5211: F, t5216: F, t5234: F, t5238: F, t5242: F, t5247: F, t5250: F) -> (F, F) {
    let t5254 = t5253 * t969;
    let t5257 = -F::new(0.3109e-1) * t5208 * t405 + F::new(1.0) * t5211 * t1221 + F::new(1.0) * t3545 * t1831 - F::new(2.0) * t3550 * t5216 + F::new(1.0) * t1212 * t5234 + F::new(0.32164683177870697974e2) * t3575 * t5238 + t4684 - t4687 - t4689 + t4692 - t4721 - t4725 - F::new(0.19751789702565206229e-1) * t4732 + F::new(0.58482233974552040708e0) * t5242 * t1226 + F::new(0.58482233974552040708e0) * t3582 * t1835 - F::new(0.11696446794910408142e1) * t3585 * t5247 + F::new(0.58482233974552040708e0) * t1225 * t5250 + F::new(0.17315755899375863299e2) * t3592 * t5254;
    (t5254, t5257)
}
