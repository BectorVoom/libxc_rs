//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk575;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk576;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk577;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk578;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk579;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk580;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk581;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk582;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk583;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk584;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta83(t1751: f64, t493: f64, t1244: f64, t1729: f64, t1756: f64, t470: f64, t494: f64, t1241: f64, t1238: f64, t1721: f64, t1752: f64, t498: f64, t265: f64, t504: f64, t1256: f64, t1534: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64, t193: f64, t336: f64, t28: f64, t1409: f64, t1649: f64, t506: f64, t52: f64, t1647: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1268: f64, t1442: f64, t1458: f64, t25: f64, t1408: f64, t514: f64, t517: f64, t157: f64, t184: f64, t17: f64, t182: f64, t1298: f64, t1302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1758 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk575(t1751, t493);
        let t1760 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk576(t1244, t1729, t1756, t1758, t470, t494);
        let t1761 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk577(t1241, t1760);
        let t1763 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk578(t1238, t1721, t1752, t1761, t498);
        let t1768 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk579(t265, t504, t1256, t1534, t1659, t1673, t1699, t1701, t1705, t1763, t193, t336);
        let t1774 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk580(t28, t1409, t1534, t1649, t1768, t265, t506, t52, t1647, dens_threshold, rho1, zeta_threshold);
        let t1778 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk581(t1268, t1442, t1458);
        let t1787 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk582(t25, t28, t1408, t514, t1649, t517, t157, zeta_threshold);
        let t1788 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk583(t1787, t184);
        let (t1789, t1791, t1799) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk584(t25, t28, t17, t1788, t1787, t182, t1298, t1408, t1302, t1649, zeta_threshold);
    (t1758, t1760, t1761, t1763, t1768, t1774, t1778, t1787, t1788, t1789, t1791, t1799)
}
