//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2232/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2232(t30: f64, t265: f64, t393: f64, t107868: f64, t106638: f64, t1469: f64, t18281: f64, t2129: f64, t28998: f64, t30727: f64, t4186: f64, t45: f64, t5825: f64, t606: f64, t7594: f64, t8161: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t111797 = piecewise3(t394, 0.0_f64, t107868);
    let t111809 = piecewise3(t120, t106638, t111797 * t45 / 2.0_f64 + t30727 * t606 / 2.0_f64 + t28998 * t1469 + t8161 * t4186 + t7594 * t5825 / 2.0_f64 + t2129 * t18281 / 2.0_f64);
    t111809
}
