//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1992;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1993;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta524(t28: f64, t265: f64, t504: f64, t21076: f64, t21999: f64, t22412: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t20217: f64, t20390: f64, t506: f64, t52: f64, t5398: f64, t5669: f64, t5966: f64, t6279: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t21713: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t19451: f64, t20293: f64, t20296: f64, t20350: f64, t20698: f64, t20702: f64, t20717: f64, t20720: f64, t4028: f64, t510: f64, t513: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6295: f64, t6468: f64, t652: f64, t7458: f64, t3: f64, t1458: f64, t5456: f64, t5493: f64, t1401: f64, t16524: f64, t20162: f64, t20347: f64, t3941: f64, t5371: f64, t576: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22414, t22424) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1992(t28, t265, t504, t21076, t21999, t22412, t1409, t1534, t1649, t1768, t20217, t20390, t506, t52, t5398, t5669, t5966, t6279, dens_threshold, rho1, zeta_threshold);
        let (t22425, t22430) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1993(t21713, t22424, t113, t1442, t1459, t1774, t1778, t1849, t19451, t20293, t20296, t20350, t20698, t20702, t20717, t20720, t4028, t510, t513, t5450, t5457, t5460, t5494, t574, t6287, t6295, t6468, t652, t7458);
        let (t22431, t22445, t22448, t22453) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1994(t22430, t3, t1458, t5456, t5493, t1401, t16524, t20162, t20347, t3941, t5371, t576, t577);
    (t22414, t22425, t22430, t22431, t22445, t22448, t22453)
}
