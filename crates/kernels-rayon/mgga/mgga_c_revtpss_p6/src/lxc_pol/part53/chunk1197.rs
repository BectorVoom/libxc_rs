//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1197/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1197(t33: f64, t265: f64, t502: f64, t127236: f64, t127287: f64, t127181: f64, t1469: f64, t32089: f64, t33897: f64, t4186: f64, t57: f64, t606: f64, t8553: f64, t28189: f64, t8568: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t127288 = t127236 + t127287;
    let t127289 = piecewise3(t503, 0.0_f64, t127181);
    let t127296 = piecewise3(t400, t127288, t127289 * t57 / 2.0_f64 - t32089 * t1469 / 2.0_f64 - t33897 * t606 / 2.0_f64 - t8553 * t4186 / 2.0_f64);
    let t127299 = t8568 * t28189;
    (t127296, t127299)
}
