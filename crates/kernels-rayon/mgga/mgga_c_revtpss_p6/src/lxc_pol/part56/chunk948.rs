//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 948/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk948(t30: f64, t265: f64, t393: f64, t32268: f64, t32275: f64, t32237: f64, t8477: f64, t32058: f64, t31882: f64, t45: f64, t606: f64, t8752: f64, t10301: f64, t8736: f64, t10309: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t32710 = t32268 * t32275;
    let t32719 = t8477 * t32237;
    let t32785 = piecewise3(t394, 0.0_f64, t32058);
    let t32790 = piecewise3(t120, t31882, t32785 * t45 / 2.0_f64 + t8752 * t606 / 2.0_f64);
    let t32795 = t10301 * t8736;
    let t32798 = t10309 * t8736;
    (t32710, t32719, t32785, t32790, t32795, t32798)
}
