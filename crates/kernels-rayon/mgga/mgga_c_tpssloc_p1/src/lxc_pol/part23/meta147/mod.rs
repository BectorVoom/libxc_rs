//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta147 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk690;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk691;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk692;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk693;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk694;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta147(t1055: f64, t5943: f64, t1052: f64, t1635: f64, t388: f64, t4557: f64, t4660: f64, t5849: f64, t5851: f64, t5915: f64, t5920: f64, t1637: f64, t1070: f64, t193: f64, t3216: f64, t336: f64, t5691: f64, t5693: f64, t5697: f64, t5729: f64, t5732: f64, t5798: f64, t5800: f64, t5802: f64, t5806: f64, t5810: f64, t5814: f64, t25: f64, t265: f64, t394: f64, t5669: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t396: f64, t40: f64, t5397: f64, t5398: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t3242: f64, t5392: f64, t3240: f64, t123: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5944, t5946, t5950) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk690(t1055, t5943, t1052, t1635, t388, t4557, t4660, t5849, t5851, t5915, t5920, t1637);
        let t5954 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk691(t1070, t193, t3216, t336, t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814, t5946, t5950);
        let (t5955, t5962) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk692(t25, t265, t394, t5669, t5954, t1408, t1409, t1534, t1642, t396, t40, t5397, t5398, dens_threshold, rho0, zeta_threshold);
        let t5966 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk693(t5397);
        let t5971 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk694(t3242, t5392);
        let (t5972, t5973, t5975) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk695(t3240, t5971, t123, t3247, t5392);
    (t5944, t5946, t5950, t5955, t5962, t5966, t5971, t5972, t5973, t5975)
}
