//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta73 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk469;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk470;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk471;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk472;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk473;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta73<F: Float>(t560: F, t225: F, t545: F, t555: F, t869: F, t689: F, t546: F, t786: F, t72: F, t686: F, t1385: F, t1399: F, t1419: F, t213: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1425, t1426) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk469::<F>(t560);
        let t1427 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk470::<F>(t1426, t225);
        let (t1428, t1429, t1431, t1432) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk471::<F>(t545, t555, t869, t689, t546, t786);
        let (t1433, t1436, t1437) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk472::<F>(t555, t72, t1432, t686, t1385);
        let t1444 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk473::<F>(t1399, t1437, t1419, t546, t1431, t1436, t213, t820);
        let t1445 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk474::<F>(t1427, t1444);
    (t1425, t1426, t1427, t1428, t1429, t1431, t1432, t1433, t1436, t1437, t1444, t1445)
}
