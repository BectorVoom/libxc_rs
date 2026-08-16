//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 538/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk538(t1197: f64, t240: f64, t1213: f64, t1550: f64, t3633: f64, t3636: f64, t3643: f64, t3674: f64, t3682: f64, t3689: f64, t3699: f64, t3718: f64, t3726: f64, t4482: f64) -> (f64, f64) {
    let t4486 = t240 * t1197;
    let t4495 = -t3633 + t3636 - t3643 + t3674 + t3682 + t240 * t4482 + 0.19751789702565206229e-1_f64 * t240 * t3689 - 0.11696446794910408142e1_f64 * t4486 * t1213 + 0.11696446794910408142e1_f64 * t1550 * t3699 - 0.58482233974552040708e0_f64 * t1550 * t3718 - 0.17315755899375863299e2_f64 * t1550 * t3726;
    (t4486, t4495)
}
