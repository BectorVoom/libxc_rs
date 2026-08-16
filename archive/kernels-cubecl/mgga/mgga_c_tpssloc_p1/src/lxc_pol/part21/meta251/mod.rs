//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1464;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1465;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1466;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1467;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1468;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta251<F: Float>(t1241: F, t6267: F, t1238: F, t1761: F, t4945: F, t498: F, t5055: F, t6151: F, t6153: F, t6239: F, t6244: F, t1763: F, t1256: F, t193: F, t336: F, t3640: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6092: F, t6094: F, t6096: F, t6100: F, t6104: F, t6108: F, t28: F, t265: F, t504: F, t5669: F, t1409: F, t1534: F, t1649: F, t1768: F, t506: F, t52: F, t5398: F, t5966: F, dens_threshold: F, rho1: F, zeta_threshold: F, t5962: F, t1268: F, t1458: F, t4028: F, t5450: F, t5456: F, t5493: F, t88: F, t5155: F, t5158: F, t1799: F, t5122: F, t5169: F, t1408: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6268 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1464::<F>(t1241, t6267);
        let (t6270, t6274, t6278) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1465::<F>(t1238, t1761, t4945, t498, t5055, t6151, t6153, t6239, t6244, t6268, t1763, t1256, t193, t336, t3640, t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108);
        let (t6279, t6286) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1466::<F>(t28, t265, t504, t5669, t6278, t1409, t1534, t1649, t1768, t506, t52, t5398, t5966, dens_threshold, rho1, zeta_threshold);
        let t6287 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1467::<F>(t5962, t6286);
        let (t6295, t6299, t6300, t6301, t6304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1468::<F>(t1268, t1458, t4028, t5450, t5456, t5493, t88, t5155, t5158, t1799, t5122, t5169);
        let t6305 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1469::<F>(t1408);
    (t6268, t6270, t6274, t6279, t6287, t6295, t6299, t6300, t6301, t6304, t6305)
}
