//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 368/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk368(t30: f64, t265: f64, t393: f64, t1102: f64, t1587: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64, t1699: f64, t198: f64, t336: f64, t1468: f64, t1469: f64, t395: f64, t45: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t1704 = piecewise3(t394, t1102 * t1699 * t198 * t336 - t1598 + t1612 + t1638 + t1640 - t1644, t1587);
    let t1709 = piecewise3(t120, t265 * t1468 / 2.0_f64 + t1587 * t30 / 2.0_f64, t395 * t1469 / 2.0_f64 + t1704 * t45 / 2.0_f64);
    (t1704, t1709)
}
