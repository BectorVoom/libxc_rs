//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1073;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1074;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta294<F: Float>(t3698: F, t3701: F, t12125: F, t12128: F, t12131: F, t12133: F, t12135: F, t12137: F, t12139: F, t12141: F, t12143: F, t1307: F, t3719: F, t3734: F, t3914: F, t3918: F, t3919: F, t5126: F, t5160: F, t6999: F, t9853: F, t9859: F, t12465: F, t12474: F, t12476: F, t3652: F, t671: F, t1266: F, t2363: F, t113: F, t11968: F, t11972: F, t1271: F, t1393: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t3660: F, t3929: F, t4034: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F, t9347: F, t9348: F, t9351: F, t9419: F, t3: F, t112: F, t3931: F, t111: F, t1395: F, t2319: F, t1401: F, t3938: F, t3941: F, t576: F, t577: F, t9416: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12477, t12490) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1073::<F>(t3698, t3701, t12125, t12128, t12131, t12133, t12135, t12137, t12139, t12141, t12143, t1307, t3719, t3734, t3914, t3918, t3919, t5126, t5160, t6999, t9853, t9859);
        let (t12492, t12504, t12507, t12512) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1074::<F>(t12465, t12474, t12476, t12490, t3652, t671, t1266, t2363, t113, t11968, t11972, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3660, t3929, t4034, t510, t513, t574, t650, t652, t672, t9347, t9348, t9351, t9419);
        let (t12513, t12521, t12524, t12529, t12532, t12537) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1075::<F>(t12512, t3, t112, t3931, t111, t1395, t2319, t671, t2363, t1401, t3938, t3941, t576, t577, t9416);
    (t12477, t12492, t12504, t12507, t12512, t12513, t12521, t12524, t12529, t12532, t12537)
}
