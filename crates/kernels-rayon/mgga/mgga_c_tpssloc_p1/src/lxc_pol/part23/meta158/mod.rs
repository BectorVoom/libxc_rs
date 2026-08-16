//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk732;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk733;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk734;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk735;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk736;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta158(t1241: f64, t6267: f64, t1238: f64, t1761: f64, t4945: f64, t498: f64, t5055: f64, t6151: f64, t6153: f64, t6239: f64, t6244: f64, t1763: f64, t1256: f64, t193: f64, t336: f64, t3640: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6092: f64, t6094: f64, t6096: f64, t6100: f64, t6104: f64, t6108: f64, t28: f64, t265: f64, t504: f64, t5669: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t506: f64, t52: f64, t5398: f64, t5966: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t5962: f64, t1268: f64, t1458: f64, t4028: f64, t5450: f64, t5456: f64, t5493: f64, t88: f64, t5155: f64, t5158: f64, t1799: f64, t5122: f64, t5169: f64, t1408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6268, t6270, t6274) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk732(t1241, t6267, t1238, t1761, t4945, t498, t5055, t6151, t6153, t6239, t6244, t1763);
        let t6278 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk733(t1256, t193, t336, t3640, t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108, t6270, t6274);
        let (t6279, t6286) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk734(t28, t265, t504, t5669, t6278, t1409, t1534, t1649, t1768, t506, t52, t5398, t5966, dens_threshold, rho1, zeta_threshold);
        let t6287 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk735(t5962, t6286);
        let (t6295, t6299, t6300, t6301, t6304) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk736(t1268, t1458, t4028, t5450, t5456, t5493, t88, t5155, t5158, t1799, t5122, t5169);
        let t6305 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk737(t1408);
    (t6268, t6270, t6274, t6279, t6287, t6295, t6299, t6300, t6301, t6304, t6305)
}
