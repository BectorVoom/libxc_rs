//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 907/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk907(t30: f64, t265: f64, t393: f64, t6084: f64, t6404: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t395: f64, t45: f64, t5824: f64, t5825: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t6405 = piecewise3(t394, t6404, t6084);
    let t6412 = piecewise3(t120, t6084 * t30 / 2.0_f64 + t1587 * t1468 + t265 * t5824 / 2.0_f64, t6405 * t45 / 2.0_f64 + t1704 * t1469 + t395 * t5825 / 2.0_f64);
    (t6405, t6412)
}
