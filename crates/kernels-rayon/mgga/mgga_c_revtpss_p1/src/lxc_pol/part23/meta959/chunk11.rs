//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3232/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3232(t33: f64, t265: f64, t502: f64, t77472: f64, t81153: f64, t81318: f64, t81350: f64, t81583: f64, t81615: f64, t81642: f64, t84999: f64, t85010: f64, t1113: f64, t1304: f64, t1469: f64, t1587: f64, t1711: f64, t18281: f64, t1837: f64, t18884: f64, t20256: f64, t21645: f64, t22671: f64, t22783: f64, t23436: f64, t25032: f64, t4186: f64, t4560: f64, t504: f64, t5509: f64, t57: f64, t5825: f64, t606: f64, t6416: f64, t6757: f64, t76397: f64, t77481: f64, t81123: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t85014 = piecewise3(t503, t81153 + t81318 + t81350 + t81583 + t81615 + t81642 + t84999 + t85010, t77472);
    let t85032 = piecewise3(t400, t77472 * t33 / 2.0_f64 + t23436 * t1113 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t18884 * t1711 - t77481 + 3.0_f64 / 2.0_f64 * t4560 * t6416 + 3.0_f64 / 2.0_f64 * t1587 * t20256 + t895 * t22783 / 2.0_f64 + t265 * t81123 / 2.0_f64, t85014 * t57 / 2.0_f64 - t25032 * t606 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t21645 * t1469 - 3.0_f64 / 2.0_f64 * t6757 * t4186 - 3.0_f64 / 2.0_f64 * t5509 * t5825 - 3.0_f64 / 2.0_f64 * t1837 * t18281 - t1304 * t22671 / 2.0_f64 - t504 * t76397 / 2.0_f64);
    t85032
}
