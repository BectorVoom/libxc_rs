//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1992;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1993;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta524<F: Float>(t28: F, t265: F, t504: F, t21076: F, t21999: F, t22412: F, t1409: F, t1534: F, t1649: F, t1768: F, t20217: F, t20390: F, t506: F, t52: F, t5398: F, t5669: F, t5966: F, t6279: F, dens_threshold: F, rho1: F, zeta_threshold: F, t21713: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t19451: F, t20293: F, t20296: F, t20350: F, t20698: F, t20702: F, t20717: F, t20720: F, t4028: F, t510: F, t513: F, t5450: F, t5457: F, t5460: F, t5494: F, t574: F, t6287: F, t6295: F, t6468: F, t652: F, t7458: F, t3: F, t1458: F, t5456: F, t5493: F, t1401: F, t16524: F, t20162: F, t20347: F, t3941: F, t5371: F, t576: F, t577: F) -> (F, F, F, F, F, F, F) {
        let (t22414, t22424) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1992::<F>(t28, t265, t504, t21076, t21999, t22412, t1409, t1534, t1649, t1768, t20217, t20390, t506, t52, t5398, t5669, t5966, t6279, dens_threshold, rho1, zeta_threshold);
        let (t22425, t22430) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1993::<F>(t21713, t22424, t113, t1442, t1459, t1774, t1778, t1849, t19451, t20293, t20296, t20350, t20698, t20702, t20717, t20720, t4028, t510, t513, t5450, t5457, t5460, t5494, t574, t6287, t6295, t6468, t652, t7458);
        let (t22431, t22445, t22448, t22453) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1994::<F>(t22430, t3, t1458, t5456, t5493, t1401, t16524, t20162, t20347, t3941, t5371, t576, t577);
    (t22414, t22425, t22430, t22431, t22445, t22448, t22453)
}
