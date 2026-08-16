//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta67 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk492;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk493;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk494;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk495;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta67(t30: f64, t33: f64, t1320: f64, t521: f64, t513: f64, t605: f64, t1113: f64, t516: f64, t162: f64, zeta_threshold: f64, t189: f64, t512: f64, t520: f64, t749: f64, t187: f64, t72: f64, t757: f64, t177: f64, t762: f64, t531: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1322, t1330) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk492(t30, t33, t1320, t521, t513, t605, t1113, t516, t162, zeta_threshold);
        let t1331 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk493(t1330, t189);
        let (t1332, t1333) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk494(t1331, t512, t520, t749);
        let (t1334, t1336, t1337, t1339, t1340) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk495(t1333, t512, t1330, t187, t520, t72, t757, t177);
        let (t1342, t1343) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk496(t1340, t762, t531, t566);
    (t1322, t1330, t1331, t1332, t1333, t1334, t1336, t1337, t1339, t1340, t1342, t1343)
}
