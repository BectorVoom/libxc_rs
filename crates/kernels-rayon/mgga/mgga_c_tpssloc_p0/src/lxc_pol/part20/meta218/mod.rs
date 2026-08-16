//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta218 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1281;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1282;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1283;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1284;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1285;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta218(t1352: f64, t5335: f64, t1338: f64, t1834: f64, t5318: f64, t553: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t1814: f64, t1838: f64, t1840: f64, t3777: f64, t5230: f64, t5234: f64, t5334: f64, t5336: f64, t5339: f64, t5341: f64, t5344: f64, t544: f64, t564: f64, t1378: f64, t1375: f64, t1386: f64, t1843: f64, t3758: f64, t3882: f64, t5211: f64, t5213: f64, t5215: f64, t5217: f64, t5319: f64, t5321: f64, t5326: f64, t568: f64, t1297: f64, t1390: f64, t193: f64, t2426: f64, t2486: f64, t3819: f64, t3821: f64, t3825: f64, t3827: f64, t3832: f64, t5167: f64, t5169: f64, t5187: f64, t5263: f64, t5265: f64, t5267: f64, t5268: f64, t5269: f64, t533: f64, t5165: f64, t113: f64, t1266: f64, t1271: f64, t1393: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t2314: f64, t4026: f64, t4028: f64, t4034: f64, t4037: f64, t4073: f64, t4077: f64, t510: f64, t5107: f64, t5118: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5345, t5348) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1281(t1352, t5335, t1338, t1834);
        let (t5349, t5351, t5353) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1282(t1352, t5348, t5318, t553, t1332, t1336, t1381, t1383, t1814, t1838, t1840, t3777, t5230, t5234, t5334, t5336, t5339, t5341, t5344, t5345, t544, t564);
        let t5354 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1283(t1378, t5353);
        let t5356 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1284(t1375, t1386, t1843, t3758, t3882, t5211, t5213, t5215, t5217, t5319, t5321, t5326, t5354, t568);
        let t5360 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1285(t1297, t1390, t193, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5167, t5169, t5187, t5263, t5265, t5267, t5268, t5269, t533, t5356);
        let (t5361, t5363) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1286(t5165, t5360, t113, t1266, t1271, t1393, t1442, t1459, t1774, t1778, t1849, t2314, t4026, t4028, t4034, t4037, t4073, t4077, t510, t5107, t5118, t513, t574, t650, t652, t672);
    (t5345, t5348, t5349, t5351, t5353, t5354, t5356, t5361, t5363)
}
