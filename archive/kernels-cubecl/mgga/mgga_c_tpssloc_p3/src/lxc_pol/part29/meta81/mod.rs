//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk534;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk535;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk536;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta81<F: Float>(t1060: F, t1629: F, t1625: F, t383: F, t1058: F, t1610: F, t353: F, t384: F, t1055: F, t1052: F, t1604: F, t1626: F, t388: F, t25: F, t265: F, t394: F, t1070: F, t1534: F, t1545: F, t1559: F, t1585: F, t1587: F, t1591: F, t193: F, t336: F, t1408: F, t1409: F, t396: F, t40: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1089: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t1630, t1632, t1634, t1635, t1637) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk534::<F>(t1060, t1629, t1625, t383, t1058, t1610, t353, t384, t1055, t1052, t1604, t1626, t388);
        let (t1642, t1647) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk535::<F>(t25, t265, t394, t1070, t1534, t1545, t1559, t1585, t1587, t1591, t1637, t193, t336, t1408, t1409, t396, t40, dens_threshold, rho0, zeta_threshold);
        let t1649 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk536::<F>(t1408);
        let t1653 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk537::<F>(t1089, t1409);
    (t1630, t1632, t1634, t1635, t1637, t1642, t1647, t1649, t1653)
}
