//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 537/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk537<F: Float>(t1212: F, t3716: F, t325: F, t3722: F, t3697: F, t3725: F, t1529: F, t1538: F, t1542: F, t1543: F, t3633: F, t3636: F, t3643: F, t3674: F, t3682: F, t3689: F, t4428: F, t4431: F, t4436: F, t4438: F, t4456: F, t4461: F, t4464: F, t4468: F, t4471: F, t4472: F, t516: F) -> (F, F, F, F) {
    let t4475 = t3716 * t1212;
    let t4478 = t325 * t3722;
    let t4479 = t3697 * t3725;
    let t4482 = -F::new(0.3109e-1) * t4428 * t516 + F::new(2.0) * t4431 * t1538 - F::new(2.0) * t4436 * t4438 + F::new(1.0) * t1529 * t4456 + F::new(0.32164683177870697974e2) * t4461 * t4464 + t3633 - t3636 + t3643 - t3674 - t3682 - F::new(0.19751789702565206229e-1) * t3689 + F::new(0.11696446794910408142e1) * t4468 * t1543 - F::new(0.11696446794910408142e1) * t4471 * t4472 + F::new(0.58482233974552040708e0) * t1542 * t4475 + F::new(0.17315755899375863299e2) * t4478 * t4479;
    (t4475, t4478, t4479, t4482)
}
