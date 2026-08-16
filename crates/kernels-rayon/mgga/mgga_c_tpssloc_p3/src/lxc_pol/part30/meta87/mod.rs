//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk559;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk560;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk561;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk562;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk563;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk564;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk565;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta87(t28: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t265: f64, t506: f64, t52: f64, t1647: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1268: f64, t1442: f64, t1458: f64, t25: f64, t1408: f64, t514: f64, t517: f64, t157: f64, t184: f64, t17: f64, t182: f64, t1298: f64, t1302: f64, t210: f64, t214: f64, t1313: f64, t1315: f64, t1322: f64, t562: f64, t119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1774 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk559(t28, t1409, t1534, t1649, t1768, t265, t506, t52, t1647, dens_threshold, rho1, zeta_threshold);
        let t1778 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk560(t1268, t1442, t1458);
        let t1787 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk561(t25, t28, t1408, t514, t1649, t517, t157, zeta_threshold);
        let t1788 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk562(t1787, t184);
        let (t1789, t1791, t1799) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk563(t25, t28, t17, t1788, t1787, t182, t1298, t1408, t1302, t1649, zeta_threshold);
        let (t1804, t1807) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk564(t1799, t210, t214, t1313, t1315, t1322);
        let (t1808, t1810, t1811) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk565(t1807, t562, t119, t1799, t210);
    (t1774, t1778, t1787, t1788, t1789, t1791, t1799, t1804, t1807, t1808, t1810, t1811)
}
