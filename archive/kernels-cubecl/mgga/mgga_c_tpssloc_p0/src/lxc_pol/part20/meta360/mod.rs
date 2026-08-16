//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta360 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1684;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1685;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1686;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1687;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta360<F: Float>(t1388: F, t3698: F, t3700: F, t570: F, t11976: F, t11978: F, t11980: F, t11982: F, t11984: F, t12012: F, t12044: F, t12046: F, t12156: F, t12451: F, t1297: F, t1390: F, t193: F, t533: F, t571: F, t9457: F, t9476: F, t9484: F, t9780: F, t3914: F, t3719: F, t12048: F, t12051: F, t12053: F, t12055: F, t12057: F, t12059: F, t12085: F, t12087: F, t12090: F, t12092: F, t12094: F, t1307: F, t3918: F, t5126: F, t9789: F, t9793: F, t12098: F, t12101: F, t12103: F, t12105: F, t12107: F, t12109: F, t12112: F, t12114: F, t12116: F, t12118: F, t12121: F, t12123: F, t9797: F, t9820: F, t9824: F, t3701: F, t12125: F, t12128: F, t12131: F, t12133: F, t12135: F, t12137: F, t12139: F, t12141: F, t12143: F, t3734: F, t3919: F, t5160: F, t6999: F, t9853: F, t9859: F, t3652: F, t671: F, t1266: F, t2363: F, t113: F, t11968: F, t11972: F, t1271: F, t1393: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t3660: F, t3929: F, t4034: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F, t9347: F, t9348: F, t9351: F, t9419: F) -> (F, F, F, F, F, F, F, F) {
        let (t12458, t12461, t12465) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1684::<F>(t1388, t3698, t3700, t570, t11976, t11978, t11980, t11982, t11984, t12012, t12044, t12046, t12156, t12451, t1297, t1390, t193, t533, t571, t9457, t9476, t9484, t9780);
        let (t12466, t12474) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1685::<F>(t1390, t3914, t3719, t571, t12048, t12051, t12053, t12055, t12057, t12059, t12085, t12087, t12090, t12092, t12094, t1307, t3918, t5126, t9789, t9793);
        let t12476 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1686::<F>(t12098, t12101, t12103, t12105, t12107, t12109, t12112, t12114, t12116, t12118, t12121, t12123, t9797, t9820, t9824);
        let (t12477, t12490) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1687::<F>(t3698, t3701, t12125, t12128, t12131, t12133, t12135, t12137, t12139, t12141, t12143, t1307, t3719, t3734, t3914, t3918, t3919, t5126, t5160, t6999, t9853, t9859);
        let (t12492, t12504, t12507, t12512) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1688::<F>(t12465, t12474, t12476, t12490, t3652, t671, t1266, t2363, t113, t11968, t11972, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3660, t3929, t4034, t510, t513, t574, t650, t652, t672, t9347, t9348, t9351, t9419);
    (t12458, t12461, t12466, t12477, t12492, t12504, t12507, t12512)
}
