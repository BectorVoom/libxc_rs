//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 537/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk537(t1212: f64, t3716: f64, t325: f64, t3722: f64, t3697: f64, t3725: f64, t1529: f64, t1538: f64, t1542: f64, t1543: f64, t3633: f64, t3636: f64, t3643: f64, t3674: f64, t3682: f64, t3689: f64, t4428: f64, t4431: f64, t4436: f64, t4438: f64, t4456: f64, t4461: f64, t4464: f64, t4468: f64, t4471: f64, t4472: f64, t516: f64) -> (f64, f64, f64, f64) {
    let t4475 = t3716 * t1212;
    let t4478 = t325 * t3722;
    let t4479 = t3697 * t3725;
    let t4482 = -0.3109e-1_f64 * t4428 * t516 + 2.0_f64 * t4431 * t1538 - 2.0_f64 * t4436 * t4438 + 1.0_f64 * t1529 * t4456 + 0.32164683177870697974e2_f64 * t4461 * t4464 + t3633 - t3636 + t3643 - t3674 - t3682 - 0.19751789702565206229e-1_f64 * t3689 + 0.11696446794910408142e1_f64 * t4468 * t1543 - 0.11696446794910408142e1_f64 * t4471 * t4472 + 0.58482233974552040708e0_f64 * t1542 * t4475 + 0.17315755899375863299e2_f64 * t4478 * t4479;
    (t4475, t4478, t4479, t4482)
}
