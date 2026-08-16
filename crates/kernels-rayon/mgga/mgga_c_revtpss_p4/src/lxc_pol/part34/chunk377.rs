//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 377/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk377(t33: f64, t1469: f64, t1587: f64, t1711: f64, t1837: f64, t265: f64, t504: f64, t57: f64, t1709: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t1842 = piecewise3(t400, t1587 * t33 / 2.0_f64 + t265 * t1711 / 2.0_f64, -t504 * t1469 / 2.0_f64 + t1837 * t57 / 2.0_f64);
    let t1843 = t1709 + t1842;
    t1843
}
