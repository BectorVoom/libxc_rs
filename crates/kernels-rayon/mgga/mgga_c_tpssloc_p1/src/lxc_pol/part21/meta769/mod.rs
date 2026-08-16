//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta769 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2665;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2666;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2667;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2668;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta769(t12550: f64, t12557: f64, t12725: f64, t12734: f64, t12813: f64, t1459: f64, t1774: f64, t19451: f64, t19456: f64, t20109: f64, t2314: f64, t2323: f64, t26114: f64, t4028: f64, t4073: f64, t4077: f64, t45632: f64, t5460: f64, t5494: f64, t55934: f64, t55969: f64, t574: f64, t652: f64, t7458: f64, t9348: f64, t12823: f64, t12841: f64, t1442: f64, t15857: f64, t20143: f64, t2320: f64, t3652: f64, t4034: f64, t4037: f64, t4072: f64, t510: f64, t5107: f64, t5118: f64, t5361: f64, t5457: f64, t55946: f64, t55962: f64, t55967: f64, t6287: f64, t1266: f64, t12724: f64, t12728: f64, t12835: f64, t19289: f64, t19450: f64, t19461: f64, t19534: f64, t20100: f64, t20127: f64, t2363: f64, t2364: f64, t4026: f64, t5493: f64, t55410: f64, t55943: f64, t671: f64, t672: f64, t89: f64, t53777: f64, t53779: f64, t53782: f64, t53787: f64, t19681: f64, t2528: f64, t172: f64, t19572: f64, t763: f64, t2535: f64, t40611: f64, t6324: f64, t12477: f64, t3698: f64, t3918: f64, t39249: f64, t39256: f64, t5160: f64, t6347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t55998 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2665(t12550, t12557, t12725, t12734, t12813, t1459, t1774, t19451, t19456, t20109, t2314, t2323, t26114, t4028, t4073, t4077, t45632, t5460, t5494, t55934, t55969, t574, t652, t7458, t9348);
        let t56034 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2666(t12725, t12734, t12823, t12841, t1442, t1459, t15857, t19456, t20109, t20143, t2314, t2320, t3652, t4028, t4034, t4037, t4072, t510, t5107, t5118, t5361, t5457, t5460, t5494, t55946, t55962, t55967, t6287, t652);
        let t56075 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2667(t1266, t12724, t12728, t12835, t1774, t19289, t19450, t19451, t19461, t19534, t20100, t20127, t2314, t2363, t2364, t3652, t4026, t4028, t4034, t510, t5107, t5493, t55410, t55943, t6287, t652, t671, t672, t7458, t89);
        let (t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56106) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2668(t53777, t53779, t53782, t53787, t19681, t2528, t172, t19572, t763, t2535, t40611, t6324);
        let t56110 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2669(t12477, t3698, t3918, t39249, t39256, t5160, t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56106, t6347);
    (t55998, t56034, t56075, t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56110)
}
