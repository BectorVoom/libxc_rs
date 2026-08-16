//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1257;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1258;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1259;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1260;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta218<F: Float>(t1367: F, t6347: F, t820: F, t1315: F, t1341: F, t1363: F, t1827: F, t1831: F, t3733: F, t3762: F, t3790: F, t3803: F, t3864: F, t5220: F, t5235: F, t5238: F, t5240: F, t5255: F, t5306: F, t559: F, t6371: F, t6375: F, t6379: F, t6390: F, t6396: F, t6417: F, t6422: F, t6427: F, t539: F, t1842: F, t3887: F, t3897: F, t6388: F, t1825: F, t5348: F, t1380: F, t6415: F, t6420: F, t553: F, t1336: F, t1814: F, t1838: F, t1840: F, t5234: F, t544: F, t564: F, t6378: F, t1378: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6431 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1257::<F>(t1367, t6347, t820);
        let t6434 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1258::<F>(t1315, t1341, t1363, t1827, t1831, t3733, t3762, t3790, t3803, t3864, t5220, t5235, t5238, t5240, t5255, t5306, t559, t6371, t6375, t6379, t6390, t6396, t6417, t6422, t6427, t6431);
        let (t6435, t6439, t6440) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1259::<F>(t539, t6434, t1842, t3887);
        let (t6448, t6451, t6454, t6456, t6458, t6460) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1260::<F>(t3897, t6388, t1825, t5348, t1380, t6415, t6420, t553, t6434, t1336, t1814, t1838, t1840, t5234, t544, t564, t6378);
        let t6461 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1261::<F>(t1378, t6460);
    (t6431, t6434, t6435, t6439, t6440, t6448, t6451, t6454, t6456, t6458, t6460, t6461)
}
