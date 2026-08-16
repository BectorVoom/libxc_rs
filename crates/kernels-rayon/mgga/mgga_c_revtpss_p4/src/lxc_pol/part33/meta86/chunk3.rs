//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 551/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk551(t33: f64, t265: f64, t502: f64, t1300: f64, t1587: f64, t1721: f64, t1735: f64, t1761: f64, t1763: f64, t1767: f64, t1832: f64, t198: f64, t336: f64, t1469: f64, t1711: f64, t504: f64, t57: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t1837 = piecewise3(t503, t1300 * t1832 * t198 * t336 - t1721 + t1735 + t1761 + t1763 - t1767, t1587);
    let t1842 = piecewise3(t400, t1587 * t33 / 2.0_f64 + t265 * t1711 / 2.0_f64, -t504 * t1469 / 2.0_f64 + t1837 * t57 / 2.0_f64);
    (t1837, t1842)
}
