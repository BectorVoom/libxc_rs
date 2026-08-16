//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1684;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1685;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1686;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1687;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta360(t1388: f64, t3698: f64, t3700: f64, t570: f64, t11976: f64, t11978: f64, t11980: f64, t11982: f64, t11984: f64, t12012: f64, t12044: f64, t12046: f64, t12156: f64, t12451: f64, t1297: f64, t1390: f64, t193: f64, t533: f64, t571: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64, t3914: f64, t3719: f64, t12048: f64, t12051: f64, t12053: f64, t12055: f64, t12057: f64, t12059: f64, t12085: f64, t12087: f64, t12090: f64, t12092: f64, t12094: f64, t1307: f64, t3918: f64, t5126: f64, t9789: f64, t9793: f64, t12098: f64, t12101: f64, t12103: f64, t12105: f64, t12107: f64, t12109: f64, t12112: f64, t12114: f64, t12116: f64, t12118: f64, t12121: f64, t12123: f64, t9797: f64, t9820: f64, t9824: f64, t3701: f64, t12125: f64, t12128: f64, t12131: f64, t12133: f64, t12135: f64, t12137: f64, t12139: f64, t12141: f64, t12143: f64, t3734: f64, t3919: f64, t5160: f64, t6999: f64, t9853: f64, t9859: f64, t3652: f64, t671: f64, t1266: f64, t2363: f64, t113: f64, t11968: f64, t11972: f64, t1271: f64, t1393: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t3660: f64, t3929: f64, t4034: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64, t9347: f64, t9348: f64, t9351: f64, t9419: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12458, t12461, t12465) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1684(t1388, t3698, t3700, t570, t11976, t11978, t11980, t11982, t11984, t12012, t12044, t12046, t12156, t12451, t1297, t1390, t193, t533, t571, t9457, t9476, t9484, t9780);
        let (t12466, t12474) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1685(t1390, t3914, t3719, t571, t12048, t12051, t12053, t12055, t12057, t12059, t12085, t12087, t12090, t12092, t12094, t1307, t3918, t5126, t9789, t9793);
        let t12476 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1686(t12098, t12101, t12103, t12105, t12107, t12109, t12112, t12114, t12116, t12118, t12121, t12123, t9797, t9820, t9824);
        let (t12477, t12490) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1687(t3698, t3701, t12125, t12128, t12131, t12133, t12135, t12137, t12139, t12141, t12143, t1307, t3719, t3734, t3914, t3918, t3919, t5126, t5160, t6999, t9853, t9859);
        let (t12492, t12504, t12507, t12512) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1688(t12465, t12474, t12476, t12490, t3652, t671, t1266, t2363, t113, t11968, t11972, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3660, t3929, t4034, t510, t513, t574, t650, t652, t672, t9347, t9348, t9351, t9419);
    (t12458, t12461, t12466, t12477, t12492, t12504, t12507, t12512)
}
