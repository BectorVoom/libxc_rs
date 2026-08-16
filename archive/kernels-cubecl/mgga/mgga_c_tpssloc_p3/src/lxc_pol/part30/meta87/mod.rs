//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk559;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk560;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk561;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk562;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk563;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk564;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk565;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta87<F: Float>(t28: F, t1409: F, t1534: F, t1649: F, t1768: F, t265: F, t506: F, t52: F, t1647: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1268: F, t1442: F, t1458: F, t25: F, t1408: F, t514: F, t517: F, t157: F, t184: F, t17: F, t182: F, t1298: F, t1302: F, t210: F, t214: F, t1313: F, t1315: F, t1322: F, t562: F, t119: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1774 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk559::<F>(t28, t1409, t1534, t1649, t1768, t265, t506, t52, t1647, dens_threshold, rho1, zeta_threshold);
        let t1778 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk560::<F>(t1268, t1442, t1458);
        let t1787 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk561::<F>(t25, t28, t1408, t514, t1649, t517, t157, zeta_threshold);
        let t1788 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk562::<F>(t1787, t184);
        let (t1789, t1791, t1799) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk563::<F>(t25, t28, t17, t1788, t1787, t182, t1298, t1408, t1302, t1649, zeta_threshold);
        let (t1804, t1807) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk564::<F>(t1799, t210, t214, t1313, t1315, t1322);
        let (t1808, t1810, t1811) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk565::<F>(t1807, t562, t119, t1799, t210);
    (t1774, t1778, t1787, t1788, t1789, t1791, t1799, t1804, t1807, t1808, t1810, t1811)
}
