//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 796/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk796(t30: f64, t265: f64, t393: f64, t2838: f64, t3339: f64, t1106: f64, t2257: f64, t2258: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t3340 = piecewise3(t394, t3339, t2838);
    let t3347 = piecewise3(t120, t2838 * t30 / 2.0_f64 + t895 * t605 + t265 * t2257 / 2.0_f64, t3340 * t45 / 2.0_f64 + t1106 * t606 + t395 * t2258 / 2.0_f64);
    (t3340, t3347)
}
