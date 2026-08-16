//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2201;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2202;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta613(t35761: f64, t35577: f64, t112: f64, t12512: f64, t111: f64, t3931: f64, t16546: f64, t576: f64, t16506: f64, t580: f64, t2319: f64, t4025: f64, t2311: f64, t671: f64, t11968: f64, t1266: f64, t12724: f64, t12728: f64, t12835: f64, t12841: f64, t1442: f64, t1459: f64, t15857: f64, t1774: f64, t2312: f64, t3652: f64, t4026: f64, t4034: f64, t4037: f64, t510: f64, t5107: f64, t650: f64, t9347: f64, t9348: f64, t9351: f64, t12723: f64, t2363: f64, t649: f64, t89: f64, t9416: f64, t12492: f64, t12557: f64, t12725: f64, t12734: f64, t12813: f64, t12816: f64, t12823: f64, t1393: f64, t1458: f64, t1778: f64, t1849: f64, t19456: f64, t2314: f64, t2364: f64, t652: f64, t672: f64, t9419: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45460, t45496, t45557, t45560, t45584, t45588, t45590) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2201(t35761, t35577, t112, t12512, t111, t3931, t16546, t576, t16506, t580, t2319, t4025);
        let (t45602, t45616) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2202(t2311, t671, t11968, t1266, t12724, t12728, t12835, t12841, t1442, t1459, t15857, t1774, t2312, t3652, t4026, t4034, t4037, t45590, t510, t5107, t650, t9347, t9348, t9351);
        let (t45632, t45637, t45648) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2203(t111, t12723, t2363, t649, t89, t9416, t11968, t12492, t12557, t1266, t12725, t12734, t12813, t12816, t12823, t12835, t1393, t1458, t1459, t1778, t1849, t19456, t2314, t2364, t4037, t652, t672, t9419);
    (t45460, t45496, t45557, t45560, t45584, t45588, t45590, t45602, t45616, t45632, t45637, t45648)
}
