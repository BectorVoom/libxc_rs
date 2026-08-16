//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta67 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk495;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk496;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk497;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk498;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk499;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk500;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta67<F: Float>(t1340: F, t762: F, t531: F, t566: F, t513: F, t30: F, t605: F, t516: F, zeta_threshold: F, t33: F, t1113: F, t212: F, t555: F, t225: F, t561: F, t689: F, t556: F, t786: F, t72: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1342, t1343) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk495::<F>(t1340, t762, t531, t566);
        let t1344 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk496::<F>(t513);
        let (t1347, t1348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk497::<F>(t30, t1344, t605, t516, zeta_threshold);
        let t1353 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk498::<F>(t33, t1113, t1348, t1347, zeta_threshold);
        let t1357 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk499::<F>(t212, t555);
        let t1358 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk500::<F>(t225, t561);
        let (t1359, t1361, t1362, t1363, t1364) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk501::<F>(t1357, t1358, t689, t556, t786, t561, t72, t686);
    (t1342, t1343, t1344, t1348, t1353, t1357, t1358, t1359, t1361, t1362, t1363, t1364)
}
