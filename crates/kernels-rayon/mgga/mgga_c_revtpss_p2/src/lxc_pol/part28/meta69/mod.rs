//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta69 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk448;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk449;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk450;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk451;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk452;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk453;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta69(t1331: f64, t512: f64, t520: f64, t749: f64, t1330: f64, t187: f64, t72: f64, t757: f64, t177: f64, t762: f64, t531: f64, t566: f64, t513: f64, t30: f64, t605: f64, t516: f64, zeta_threshold: f64, t33: f64, t1113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1332, t1333) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk448(t1331, t512, t520, t749);
        let (t1334, t1336, t1337, t1339, t1340) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk449(t1333, t512, t1330, t187, t520, t72, t757, t177);
        let (t1342, t1343) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk450(t1340, t762, t531, t566);
        let t1344 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk451(t513);
        let (t1347, t1348) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk452(t30, t1344, t605, t516, zeta_threshold);
        let t1353 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk453(t33, t1113, t1348, t1347, zeta_threshold);
    (t1332, t1333, t1334, t1336, t1337, t1339, t1340, t1342, t1343, t1344, t1348, t1353)
}
