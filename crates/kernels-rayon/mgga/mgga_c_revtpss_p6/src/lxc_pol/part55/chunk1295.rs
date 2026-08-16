//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1295/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1295(t128970: f64, t128974: f64, t128975: f64, t128977: f64, t128979: f64, t128981: f64, t128983: f64, t129353: f64, t2052: f64, t28287: f64, t28927: f64, t29337: f64, t29427: f64, t29459: f64, t32822: f64, t7357: f64, t7359: f64, t7374: f64, t8079: f64, t8233: f64, t8764: f64) -> f64 {
    let t131092 = 2.0_f64 * t129353 * t28287 - t2052 * t29337 + t28927 * t8764 - 2.0_f64 * t29427 * t7374 - 2.0_f64 * t29459 * t7359 + 3.0_f64 * t32822 * t8079 - t7357 * t8233 + t128970 - t128974 + t128975 - t128977 - t128979 - t128981 - t128983;
    t131092
}
