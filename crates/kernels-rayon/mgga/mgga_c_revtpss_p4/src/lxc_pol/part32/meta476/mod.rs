//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1710;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1711;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1712;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta476(t116: f64, t7373: f64, t1518: f64, t648: f64, t4292: f64, t94: f64, t1353: f64, t1907: f64, t30: f64, t892: f64, t4433: f64, t18875: f64, t25207: f64, t1544: f64, t605: f64, t4343: f64, t1032: f64, t1568: f64, t1955: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26733, t27123) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1710(t116, t7373, t1518, t648);
        let (t27126, t27153, t27159, t27160, t27166, t27169, t27173) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1711(t4292, t94, t1353, t1907, t30, t892, t4433, t18875, t25207, t1544, t605, t4343);
        let (t27198, t27199) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1712(t1032, t1568, t1955);
        let t27212 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1713(t27198, t867);
    (t26733, t27123, t27126, t27153, t27159, t27160, t27166, t27169, t27173, t27198, t27199, t27212)
}
