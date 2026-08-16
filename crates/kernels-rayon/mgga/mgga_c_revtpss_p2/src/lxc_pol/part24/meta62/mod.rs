//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta62 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk395;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk396;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk397;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk398;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta62(t1570: f64, t1580: f64, t213: f64, t783: f64, t791: f64, t865: f64, t1524: f64, t1533: f64, t1536: f64, t1544: f64, t198: f64, t207: f64, t679: f64, t704: f64, t751: f64, t759: f64, t764: f64, t765: f64, t892: f64, t1469: f64, t905: f64, t904: f64, t128: f64, t903: f64, t291: f64, t902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1583 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk395(t1570, t1580, t213, t783, t791, t865);
        let t1587 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk396(t1524, t1533, t1536, t1544, t1583, t198, t207, t679, t704, t751, t759, t764, t765, t892);
        let t1592 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk397(t1469, t905);
        let (t1593, t1594, t1596) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk398(t1592, t904, t128, t903);
        let (t1598, t1600) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk399(t1596, t291, t1594, t902);
    (t1583, t1587, t1592, t1593, t1594, t1596, t1598, t1600)
}
