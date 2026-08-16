//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1097;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1098;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1099;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1100;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta242(t3887: f64, t6439: f64, t3897: f64, t6388: f64, t1825: f64, t5348: f64, t1380: f64, t6415: f64, t6420: f64, t553: f64, t6434: f64, t1336: f64, t1814: f64, t1838: f64, t1840: f64, t5234: f64, t544: f64, t564: f64, t6378: f64, t1378: f64, t1375: f64, t1843: f64, t5215: f64, t5321: f64, t568: f64, t6362: f64, t6364: f64, t6435: f64, t1297: f64, t1390: f64, t193: f64, t2486: f64, t3701: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t3924: f64, t533: f64, t6324: f64, t6329: f64, t6330: f64, t6347: f64, t6399: f64, t6400: f64, t6323: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t4028: f64, t510: f64, t513: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t574: f64, t6287: f64, t6295: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6440, t6448, t6451, t6454, t6456, t6458, t6460) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1097(t3887, t6439, t3897, t6388, t1825, t5348, t1380, t6415, t6420, t553, t6434, t1336, t1814, t1838, t1840, t5234, t544, t564, t6378);
        let t6461 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1098(t1378, t6460);
        let t6463 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1099(t1375, t1843, t5215, t5321, t568, t6362, t6364, t6435, t6440, t6461);
        let t6467 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1100(t1297, t1390, t193, t2486, t3701, t3819, t3821, t3823, t3825, t3832, t3836, t3924, t533, t6324, t6329, t6330, t6347, t6399, t6400, t6463);
        let (t6468, t6470) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1101(t6323, t6467, t113, t1442, t1459, t1774, t1778, t1849, t4028, t510, t513, t5450, t5457, t5460, t5494, t574, t6287, t6295, t652);
    (t6440, t6448, t6451, t6454, t6456, t6458, t6460, t6461, t6463, t6468, t6470)
}
