//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta237 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1033;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1034;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1035;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1036;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1037;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta237<F: Float>(t1352: F, t5335: F, t1338: F, t1834: F, t5318: F, t553: F, t1332: F, t1336: F, t1381: F, t1383: F, t1814: F, t1838: F, t1840: F, t3777: F, t5230: F, t5234: F, t5334: F, t5336: F, t5339: F, t5341: F, t5344: F, t544: F, t564: F, t1378: F, t1375: F, t1386: F, t1843: F, t3758: F, t3882: F, t5211: F, t5213: F, t5215: F, t5217: F, t5319: F, t5321: F, t5326: F, t568: F, t1297: F, t1390: F, t193: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5167: F, t5169: F, t5187: F, t5263: F, t5265: F, t5267: F, t5268: F, t5269: F, t533: F, t5165: F, t113: F, t1266: F, t1271: F, t1393: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t2314: F, t4026: F, t4028: F, t4034: F, t4037: F, t4073: F, t4077: F, t510: F, t5107: F, t5118: F, t513: F, t574: F, t650: F, t652: F, t672: F, t3: F, t112: F, t1851: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5345, t5348, t5349, t5351, t5353) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1033::<F>(t1352, t5335, t1338, t1834, t5318, t553, t1332, t1336, t1381, t1383, t1814, t1838, t1840, t3777, t5230, t5234, t5334, t5336, t5339, t5341, t5344, t544, t564);
        let t5354 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1034::<F>(t1378, t5353);
        let t5356 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1035::<F>(t1375, t1386, t1843, t3758, t3882, t5211, t5213, t5215, t5217, t5319, t5321, t5326, t5354, t568);
        let t5360 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1036::<F>(t1297, t1390, t193, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5167, t5169, t5187, t5263, t5265, t5267, t5268, t5269, t533, t5356);
        let (t5361, t5363) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1037::<F>(t5165, t5360, t113, t1266, t1271, t1393, t1442, t1459, t1774, t1778, t1849, t2314, t4026, t4028, t4034, t4037, t4073, t4077, t510, t5107, t5118, t513, t574, t650, t652, t672);
        let (t5364, t5371) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1038::<F>(t3, t5363, t112, t1851);
    (t5345, t5348, t5349, t5351, t5353, t5354, t5356, t5361, t5363, t5364, t5371)
}
