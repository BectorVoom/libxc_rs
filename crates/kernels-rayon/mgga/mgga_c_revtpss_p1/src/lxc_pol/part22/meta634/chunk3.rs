//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2560/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2560(t30: f64, t265: f64, t393: f64, t18884: f64, t19141: f64, t20234: f64, t1106: f64, t1468: f64, t1469: f64, t1704: f64, t18280: f64, t18281: f64, t18892: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t5028: f64, t5824: f64, t5825: f64, t605: f64, t606: f64, t6084: f64, t6405: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t20236 = piecewise3(t394, t19141 + t20234, t18884);
    let t20248 = piecewise3(t120, t18884 * t30 / 2.0_f64 + t6084 * t605 / 2.0_f64 + t4560 * t1468 + t18892 + t895 * t5824 / 2.0_f64 + t265 * t18280 / 2.0_f64, t20236 * t45 / 2.0_f64 + t6405 * t606 / 2.0_f64 + t5028 * t1469 + t1704 * t4186 + t1106 * t5825 / 2.0_f64 + t395 * t18281 / 2.0_f64);
    (t20236, t20248)
}
