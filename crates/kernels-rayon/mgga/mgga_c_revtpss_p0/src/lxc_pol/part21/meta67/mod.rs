//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta67 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk495;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk496;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk497;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk498;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk499;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk500;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta67(t1340: f64, t762: f64, t531: f64, t566: f64, t513: f64, t30: f64, t605: f64, t516: f64, zeta_threshold: f64, t33: f64, t1113: f64, t212: f64, t555: f64, t225: f64, t561: f64, t689: f64, t556: f64, t786: f64, t72: f64, t686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1342, t1343) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk495(t1340, t762, t531, t566);
        let t1344 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk496(t513);
        let (t1347, t1348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk497(t30, t1344, t605, t516, zeta_threshold);
        let t1353 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk498(t33, t1113, t1348, t1347, zeta_threshold);
        let t1357 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk499(t212, t555);
        let t1358 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk500(t225, t561);
        let (t1359, t1361, t1362, t1363, t1364) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk501(t1357, t1358, t689, t556, t786, t561, t72, t686);
    (t1342, t1343, t1344, t1348, t1353, t1357, t1358, t1359, t1361, t1362, t1363, t1364)
}
