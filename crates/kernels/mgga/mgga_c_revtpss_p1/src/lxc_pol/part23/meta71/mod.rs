//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta71 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk496;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk497;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk498;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk499;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk500;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta71<F: Float>(t545: F, t555: F, t869: F, t689: F, t546: F, t786: F, t72: F, t686: F, t1385: F, t1399: F, t1419: F, t213: F, t820: F, t1427: F, t1361: F, t1366: F, t1421: F, t1424: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1428, t1429, t1431, t1432) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk496::<F>(t545, t555, t869, t689, t546, t786);
        let t1433 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk497::<F>(t555, t72);
        let (t1436, t1437) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk498::<F>(t1432, t1433, t686, t1385, t555);
        let (t1438, t1441, t1444) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk499::<F>(t1399, t1437, t1419, t546, t1431, t1436, t213, t820);
        let t1445 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk500::<F>(t1427, t1444);
        let t1448 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk501::<F>(t1361, t1366, t1421, t1424, t1445, t213);
    (t1428, t1429, t1431, t1432, t1433, t1436, t1437, t1438, t1441, t1444, t1445, t1448)
}
