//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 793/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk793(t2046: f64, t8589: f64, t7316: f64, t7318: f64, t8556: f64, t8558: f64, t8562: f64, t8567: f64, t8572: f64, t8574: f64, t8576: f64, t8578: f64, t8580: f64, t8582: f64, t8584: f64, t8586: f64) -> f64 {
    let t8590 = t2046 * t8589;
    let t8594 = 0.52413487149340253447e-3_f64 * t8556 - 0.31448092289604152068e-3_f64 * t8558 - 0.31448092289604152068e-3_f64 * t8562 - 0.31448092289604152068e-3_f64 * t8567 - 0.20965394859736101379e-3_f64 * t8572 - 0.42874018118069736972e-3_f64 * t8574 - 0.42874018118069736972e-3_f64 * t8576 + 0.47172138434406228102e-3_f64 * t8578 - 0.94344276868812456204e-3_f64 * t8580 - 0.10718504529517434243e-3_f64 * t8582 - t8584 / 96.0_f64 - t8586 / 96.0_f64 - t8590 / 128.0_f64 + 11.0_f64 / 384.0_f64 * t7316 + 11.0_f64 / 1152.0_f64 * t7318;
    t8594
}
