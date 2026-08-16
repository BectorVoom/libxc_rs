//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1486;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1487;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1488;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1489;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta255(t550: f64, t6387: f64, t1343: f64, t820: f64, t3870: f64, t6330: f64, t1367: f64, t6347: f64, t1315: f64, t1341: f64, t1363: f64, t1827: f64, t1831: f64, t3733: f64, t3762: f64, t3790: f64, t3803: f64, t3864: f64, t5220: f64, t5235: f64, t5238: f64, t5240: f64, t5255: f64, t5306: f64, t559: f64, t6371: f64, t6375: f64, t6379: f64, t6390: f64, t6396: f64, t6417: f64, t539: f64, t1842: f64, t3887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6420, t6422) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1486(t550, t6387, t1343, t820);
        let t6427 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1487(t3870, t6330, t820);
        let t6431 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1488(t1367, t6347, t820);
        let t6434 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1489(t1315, t1341, t1363, t1827, t1831, t3733, t3762, t3790, t3803, t3864, t5220, t5235, t5238, t5240, t5255, t5306, t559, t6371, t6375, t6379, t6390, t6396, t6417, t6422, t6427, t6431);
        let (t6435, t6439, t6440) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1490(t539, t6434, t1842, t3887);
    (t6420, t6422, t6427, t6431, t6434, t6435, t6439, t6440)
}
