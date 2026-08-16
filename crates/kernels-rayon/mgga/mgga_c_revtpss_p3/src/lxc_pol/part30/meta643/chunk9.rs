//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2258/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2258(t33: f64, t265: f64, t502: f64, t100927: f64, t105696: f64, t101107: f64, t13312: f64, t1469: f64, t2159: f64, t2258: f64, t27048: f64, t29329: f64, t4186: f64, t57: f64, t606: f64, t7677: f64, t8227: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t105697 = piecewise3(t503, t105696, t100927);
    let t105709 = piecewise3(t400, t101107, t105697 * t57 / 2.0_f64 - t29329 * t606 - t8227 * t2258 / 2.0_f64 - t27048 * t1469 / 2.0_f64 - t7677 * t4186 - t2159 * t13312 / 2.0_f64);
    t105709
}
