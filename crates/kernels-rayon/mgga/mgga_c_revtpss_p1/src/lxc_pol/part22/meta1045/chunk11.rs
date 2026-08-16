//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3670/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3670(t58207: f64, t68454: f64, t68529: f64, t68532: f64, t68535: f64, t68538: f64, t68540: f64, t68543: f64, t68546: f64, t68548: f64, t68550: f64, t68553: f64, t68556: f64, t68559: f64, t68561: f64) -> f64 {
    let t69329 = 0.55570666666666666666e0_f64 * t68529 - 0.10805407407407407407e0_f64 * t68532 + 0.41678e0_f64 * t68535 - 0.61745185185185185187e-1_f64 * t58207 - 0.55570666666666666667e0_f64 * t68538 - 0.83356000000000000001e0_f64 * t68540 + 0.20839e0_f64 * t68543 + 0.62517e0_f64 * t68546 + 0.92617777777777777779e-1_f64 * t68548 + 0.27785333333333333334e0_f64 * t68550 - 0.69463333333333333334e-1_f64 * t68553 + 0.46308888888888888889e-1_f64 * t68556 + 0.10589175e2_f64 * t68559 - 0.6311625e0_f64 * t68561 - 0.13772666666666666667e1_f64 * t68454;
    t69329
}
