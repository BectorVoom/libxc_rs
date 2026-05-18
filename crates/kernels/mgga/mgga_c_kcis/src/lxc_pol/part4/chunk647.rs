//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 647/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk647<F: Float>(t3025: F, t971: F, t265: F, t3031: F, t3006: F, t3034: F, t1212: F, t1221: F, t1225: F, t1226: F, t2932: F, t2935: F, t2942: F, t2983: F, t2991: F, t2998: F, t3542: F, t3545: F, t3550: F, t3552: F, t3570: F, t3575: F, t3578: F, t3582: F, t3585: F, t3586: F, t405: F) -> (F, F, F, F) {
    let t3589 = t3025 * t971;
    let t3592 = t265 * t3031;
    let t3593 = t3006 * t3034;
    let t3596 = -F::new(0.3109e-1) * t3542 * t405 + F::new(2.0) * t3545 * t1221 - F::new(2.0) * t3550 * t3552 + F::new(1.0) * t1212 * t3570 + F::new(0.32164683177870697974e2) * t3575 * t3578 + t2932 - t2935 + t2942 - t2983 - t2991 - F::new(0.19751789702565206229e-1) * t2998 + F::new(0.11696446794910408142e1) * t3582 * t1226 - F::new(0.11696446794910408142e1) * t3585 * t3586 + F::new(0.58482233974552040708e0) * t1225 * t3589 + F::new(0.17315755899375863299e2) * t3592 * t3593;
    (t3589, t3592, t3593, t3596)
}
