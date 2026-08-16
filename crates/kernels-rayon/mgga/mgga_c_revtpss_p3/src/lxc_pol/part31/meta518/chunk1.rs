//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1876/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1876(t30: f64, t1469: f64, t1996: f64, t27408: f64, t27755: f64, t4186: f64, t45: f64, t606: f64, t7194: f64, t7856: f64, t33: f64, t892: f64, t4433: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t27762 = piecewise3(t120, t27408, t7194 * t1469 / 2.0_f64 + t1996 * t4186 / 2.0_f64 + t27755 * t45 / 2.0_f64 + t7856 * t606 / 2.0_f64);
    let t27763 = t892 * t33;
    let t27764 = t27763 * t4433;
    (t27762, t27763, t27764)
}
