//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta78 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk541;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk542;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk543;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk544;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk545;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk546;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk547;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk548;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta78(t1625: f64, t349: f64, t1615: f64, t381: f64, t1060: f64, t383: f64, t1058: f64, t1610: f64, t353: f64, t384: f64, t1055: f64, t1052: f64, t1604: f64, t388: f64, t265: f64, t394: f64, t1070: f64, t1534: f64, t1545: f64, t1559: f64, t1585: f64, t1587: f64, t1591: f64, t193: f64, t336: f64, t25: f64, t1408: f64, t1409: f64, t396: f64, t40: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1626, t1629) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk541(t1625, t349, t1615, t381);
        let t1630 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk542(t1060, t1629);
        let t1632 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk543(t1625, t383);
        let t1634 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk544(t1058, t1610, t1630, t1632, t353, t384);
        let t1635 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk545(t1055, t1634);
        let t1637 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk546(t1052, t1604, t1626, t1635, t388);
        let t1642 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk547(t265, t394, t1070, t1534, t1545, t1559, t1585, t1587, t1591, t1637, t193, t336);
        let (t1647, t1649) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk548(t25, t1408, t1409, t1534, t1642, t265, t396, t40, dens_threshold, rho0, zeta_threshold);
        let t1653 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk549(t1089, t1409);
    (t1626, t1629, t1630, t1632, t1634, t1635, t1637, t1642, t1647, t1649, t1653)
}
