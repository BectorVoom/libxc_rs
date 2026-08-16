//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta241 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1091;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1092;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1093;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1094;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1095;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta241<F: Float>(t550: F, t6387: F, t1343: F, t820: F, t3870: F, t6330: F, t1367: F, t6347: F, t1315: F, t1341: F, t1363: F, t1827: F, t1831: F, t3733: F, t3762: F, t3790: F, t3803: F, t3864: F, t5220: F, t5235: F, t5238: F, t5240: F, t5255: F, t5306: F, t559: F, t6371: F, t6375: F, t6379: F, t6390: F, t6396: F, t6417: F, t539: F, t1842: F) -> (F, F, F, F, F, F, F) {
        let t6420 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1091::<F>(t550, t6387);
        let t6422 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1092::<F>(t1343, t6420, t820);
        let t6427 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1093::<F>(t3870, t6330, t820);
        let t6431 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1094::<F>(t1367, t6347, t820);
        let t6434 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1095::<F>(t1315, t1341, t1363, t1827, t1831, t3733, t3762, t3790, t3803, t3864, t5220, t5235, t5238, t5240, t5255, t5306, t559, t6371, t6375, t6379, t6390, t6396, t6417, t6422, t6427, t6431);
        let (t6435, t6439) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1096::<F>(t539, t6434, t1842);
    (t6420, t6422, t6427, t6431, t6434, t6435, t6439)
}
