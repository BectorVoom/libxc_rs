//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta77 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk534;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk535;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk536;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk537;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta77(t1524: f64, t1533: f64, t1536: f64, t1544: f64, t1583: f64, t198: f64, t207: f64, t679: f64, t704: f64, t751: f64, t759: f64, t764: f64, t765: f64, t892: f64, t1469: f64, t905: f64, t904: f64, t128: f64, t903: f64, t291: f64, t902: f64, t916: f64, t923: f64, t930: f64, t141: f64, t921: f64, t929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1587 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk534(t1524, t1533, t1536, t1544, t1583, t198, t207, t679, t704, t751, t759, t764, t765, t892);
        let t1592 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk535(t1469, t905);
        let (t1593, t1594, t1596) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk536(t1592, t904, t128, t903);
        let (t1598, t1600) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk537(t1596, t291, t1594, t902);
        let (t1601, t1604, t1606, t1607, t1609) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk538(t1600, t916, t923, t1592, t930, t141, t1594, t921, t929);
    (t1587, t1592, t1593, t1594, t1596, t1598, t1600, t1601, t1604, t1606, t1607, t1609)
}
