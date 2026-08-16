//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta72 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk469;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk470;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk471;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk472;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk473;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta72(t560: f64, t225: f64, t545: f64, t555: f64, t869: f64, t689: f64, t546: f64, t786: f64, t72: f64, t686: f64, t1385: f64, t1399: f64, t1419: f64, t213: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1425, t1426) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk469(t560);
        let t1427 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk470(t1426, t225);
        let (t1428, t1429, t1431, t1432) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk471(t545, t555, t869, t689, t546, t786);
        let (t1433, t1436, t1437) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk472(t555, t72, t1432, t686, t1385);
        let t1444 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk473(t1399, t1437, t1419, t546, t1431, t1436, t213, t820);
        let t1445 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk474(t1427, t1444);
    (t1425, t1426, t1427, t1428, t1429, t1431, t1432, t1433, t1436, t1437, t1444, t1445)
}
