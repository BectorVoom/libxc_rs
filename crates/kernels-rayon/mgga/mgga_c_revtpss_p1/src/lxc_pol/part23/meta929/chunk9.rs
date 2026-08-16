//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3042/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3042(t30: f64, t265: f64, t393: f64, t77472: f64, t78403: f64, t78414: f64, t78444: f64, t78475: f64, t81075: f64, t81076: f64, t81078: f64, t81088: f64, t1106: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t18280: f64, t18281: f64, t18884: f64, t20236: f64, t22670: f64, t22671: f64, t23436: f64, t24192: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t5028: f64, t5824: f64, t5825: f64, t605: f64, t606: f64, t6405: f64, t76396: f64, t76397: f64, t77481: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t81092 = piecewise3(t394, t78403 + t78414 + t78444 + t78475 + t81075 + t81076 + t81078 + t81088, t77472);
    let t81110 = piecewise3(t120, t77472 * t30 / 2.0_f64 + t23436 * t605 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t18884 * t1468 + t77481 + 3.0_f64 / 2.0_f64 * t4560 * t5824 + 3.0_f64 / 2.0_f64 * t1587 * t18280 + t895 * t22670 / 2.0_f64 + t265 * t76396 / 2.0_f64, t81092 * t45 / 2.0_f64 + t24192 * t606 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t20236 * t1469 + 3.0_f64 / 2.0_f64 * t6405 * t4186 + 3.0_f64 / 2.0_f64 * t5028 * t5825 + 3.0_f64 / 2.0_f64 * t1704 * t18281 + t1106 * t22671 / 2.0_f64 + t395 * t76397 / 2.0_f64);
    t81110
}
