//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta67 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk492;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk493;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk494;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk495;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta67<F: Float>(t30: F, t33: F, t1320: F, t521: F, t513: F, t605: F, t1113: F, t516: F, t162: F, zeta_threshold: F, t189: F, t512: F, t520: F, t749: F, t187: F, t72: F, t757: F, t177: F, t762: F, t531: F, t566: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1322, t1330) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk492::<F>(t30, t33, t1320, t521, t513, t605, t1113, t516, t162, zeta_threshold);
        let t1331 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk493::<F>(t1330, t189);
        let (t1332, t1333) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk494::<F>(t1331, t512, t520, t749);
        let (t1334, t1336, t1337, t1339, t1340) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk495::<F>(t1333, t512, t1330, t187, t520, t72, t757, t177);
        let (t1342, t1343) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk496::<F>(t1340, t762, t531, t566);
    (t1322, t1330, t1331, t1332, t1333, t1334, t1336, t1337, t1339, t1340, t1342, t1343)
}
