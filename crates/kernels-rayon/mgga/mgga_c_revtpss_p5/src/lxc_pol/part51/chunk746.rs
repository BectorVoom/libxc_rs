//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 746/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk746(t30: f64, t265: f64, t393: f64, t207: f64, t8493: f64, t198: f64, t2411: f64, t8536: f64, t892: f64, t1102: f64, t3336: f64, t336: f64, t8527: f64, t8531: f64, t45: f64, t8498: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8539 = t207 * t8493;
    let t8542 = -t198 * t2411 * t8539 + t198 * t8536 * t892;
    let t8543 = piecewise3(t394, t1102 * t198 * t336 * t8527 - t198 * t3336 * t336 * t8531, t8542);
    let t8546 = piecewise3(t120, t8498, t8543 * t45 / 2.0_f64);
    (t8539, t8542, t8543, t8546)
}
