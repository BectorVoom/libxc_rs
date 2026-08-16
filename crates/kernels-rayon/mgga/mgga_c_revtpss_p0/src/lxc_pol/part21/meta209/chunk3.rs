//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1267/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1267(t30: f64, t265: f64, t393: f64, t4560: f64, t5027: f64, t1106: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t395: f64, t4186: f64, t45: f64, t4568: f64, t605: f64, t606: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t5028 = piecewise3(t394, t5027, t4560);
    let t5035 = piecewise3(t120, t4560 * t30 / 2.0_f64 + t1587 * t605 / 2.0_f64 + t895 * t1468 / 2.0_f64 + t4568, t1106 * t1469 / 2.0_f64 + t1704 * t606 / 2.0_f64 + t395 * t4186 / 2.0_f64 + t5028 * t45 / 2.0_f64);
    (t5028, t5035)
}
