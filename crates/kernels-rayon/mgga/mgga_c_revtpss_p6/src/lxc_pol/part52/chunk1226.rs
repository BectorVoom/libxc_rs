//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1226/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1226(t33: f64, t265: f64, t502: f64, t128061: f64, t128097: f64, t128121: f64, t128150: f64, t128183: f64, t1469: f64, t32569: f64, t34161: f64, t4186: f64, t57: f64, t606: f64, t8682: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t128186 = piecewise3(t503, 0.0_f64, t128061);
    let t128193 = piecewise3(t400, t128097 + t128121 + t128150 + t128183, t128186 * t57 / 2.0_f64 - t32569 * t1469 / 2.0_f64 - t34161 * t606 / 2.0_f64 - t8682 * t4186 / 2.0_f64);
    t128193
}
