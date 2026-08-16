//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 366/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk366(t30: f64, t265: f64, t393: f64, t1100: f64, t1102: f64, t198: f64, t336: f64, t895: f64, t912: f64, t938: f64, t978: f64, t980: f64, t985: f64, t395: f64, t45: f64, t605: f64, t606: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t1106 = piecewise3(t394, t1100 * t1102 * t198 * t336 - t912 + t938 + t978 + t980 - t985, t895);
    let t1111 = piecewise3(t120, t265 * t605 / 2.0_f64 + t895 * t30 / 2.0_f64, t1106 * t45 / 2.0_f64 + t395 * t606 / 2.0_f64);
    (t1106, t1111)
}
