//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 554/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk554(t3025: f64, t971: f64, t265: f64, t3031: f64, t3006: f64, t3034: f64, t1212: f64, t1221: f64, t1225: f64, t1226: f64, t2932: f64, t2935: f64, t2942: f64, t2983: f64, t2991: f64, t2998: f64, t3542: f64, t3545: f64, t3550: f64, t3552: f64, t3570: f64, t3575: f64, t3578: f64, t3582: f64, t3585: f64, t3586: f64, t405: f64) -> (f64, f64, f64, f64) {
    let t3589 = t3025 * t971;
    let t3592 = t265 * t3031;
    let t3593 = t3006 * t3034;
    let t3596 = -0.3109e-1_f64 * t3542 * t405 + 2.0_f64 * t3545 * t1221 - 2.0_f64 * t3550 * t3552 + 1.0_f64 * t1212 * t3570 + 0.32164683177870697974e2_f64 * t3575 * t3578 + t2932 - t2935 + t2942 - t2983 - t2991 - 0.19751789702565206229e-1_f64 * t2998 + 0.11696446794910408142e1_f64 * t3582 * t1226 - 0.11696446794910408142e1_f64 * t3585 * t3586 + 0.58482233974552040708e0_f64 * t1225 * t3589 + 0.17315755899375863299e2_f64 * t3592 * t3593;
    (t3589, t3592, t3593, t3596)
}
