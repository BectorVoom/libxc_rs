//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 165/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk165(t407: f64, t410: f64, t413: f64, t417: f64) -> (f64, f64, f64) {
    let t419 = 0.379785e1_f64 * t410 + 0.8969e0_f64 * t407 + 0.204775e0_f64 * t413 + 0.123235e0_f64 * t417;
    let t422 = 1.0_f64 + 0.16081979498692535067e2_f64 / t419;
    let t423 = f64::ln(t422);
    (t419, t422, t423)
}
