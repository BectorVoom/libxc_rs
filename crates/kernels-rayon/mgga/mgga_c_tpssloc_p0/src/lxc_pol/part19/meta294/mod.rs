//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1073;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1074;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta294(t3698: f64, t3701: f64, t12125: f64, t12128: f64, t12131: f64, t12133: f64, t12135: f64, t12137: f64, t12139: f64, t12141: f64, t12143: f64, t1307: f64, t3719: f64, t3734: f64, t3914: f64, t3918: f64, t3919: f64, t5126: f64, t5160: f64, t6999: f64, t9853: f64, t9859: f64, t12465: f64, t12474: f64, t12476: f64, t3652: f64, t671: f64, t1266: f64, t2363: f64, t113: f64, t11968: f64, t11972: f64, t1271: f64, t1393: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t3660: f64, t3929: f64, t4034: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64, t9347: f64, t9348: f64, t9351: f64, t9419: f64, t3: f64, t112: f64, t3931: f64, t111: f64, t1395: f64, t2319: f64, t1401: f64, t3938: f64, t3941: f64, t576: f64, t577: f64, t9416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12477, t12490) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1073(t3698, t3701, t12125, t12128, t12131, t12133, t12135, t12137, t12139, t12141, t12143, t1307, t3719, t3734, t3914, t3918, t3919, t5126, t5160, t6999, t9853, t9859);
        let (t12492, t12504, t12507, t12512) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1074(t12465, t12474, t12476, t12490, t3652, t671, t1266, t2363, t113, t11968, t11972, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3660, t3929, t4034, t510, t513, t574, t650, t652, t672, t9347, t9348, t9351, t9419);
        let (t12513, t12521, t12524, t12529, t12532, t12537) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1075(t12512, t3, t112, t3931, t111, t1395, t2319, t671, t2363, t1401, t3938, t3941, t576, t577, t9416);
    (t12477, t12492, t12504, t12507, t12512, t12513, t12521, t12524, t12529, t12532, t12537)
}
