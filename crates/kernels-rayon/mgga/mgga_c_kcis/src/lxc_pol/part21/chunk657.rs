//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 657/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk657(t5253: f64, t969: f64, t1212: f64, t1221: f64, t1225: f64, t1226: f64, t1831: f64, t1835: f64, t3545: f64, t3550: f64, t3575: f64, t3582: f64, t3585: f64, t3592: f64, t405: f64, t4684: f64, t4687: f64, t4689: f64, t4692: f64, t4721: f64, t4725: f64, t4732: f64, t5208: f64, t5211: f64, t5216: f64, t5234: f64, t5238: f64, t5242: f64, t5247: f64, t5250: f64) -> (f64, f64) {
    let t5254 = t5253 * t969;
    let t5257 = -0.3109e-1_f64 * t5208 * t405 + 1.0_f64 * t5211 * t1221 + 1.0_f64 * t3545 * t1831 - 2.0_f64 * t3550 * t5216 + 1.0_f64 * t1212 * t5234 + 0.32164683177870697974e2_f64 * t3575 * t5238 + t4684 - t4687 - t4689 + t4692 - t4721 - t4725 - 0.19751789702565206229e-1_f64 * t4732 + 0.58482233974552040708e0_f64 * t5242 * t1226 + 0.58482233974552040708e0_f64 * t3582 * t1835 - 0.11696446794910408142e1_f64 * t3585 * t5247 + 0.58482233974552040708e0_f64 * t1225 * t5250 + 0.17315755899375863299e2_f64 * t3592 * t5254;
    (t5254, t5257)
}
