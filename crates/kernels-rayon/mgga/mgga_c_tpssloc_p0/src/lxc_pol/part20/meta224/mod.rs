//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1299;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1300;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1301;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1302;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta224(t9287: f64, t9288: f64, t2267: f64, t607: f64, t2250: f64, t43: f64, t9258: f64, t53: f64, t54: f64, t2274: f64, t55: f64, t2585: f64, t2262: f64, t2268: f64, t2271: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t9277: f64, t33: f64, t2769: f64, t73: f64, t2291: f64, t3241: f64, t76: f64, t2298: f64, t634: f64, t638: f64, t72: f64, t2245: f64, t2252: f64, t2255: f64, t2284: f64, t2304: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64, t9247: f64, t9248: f64, t9251: f64, t9260: f64, t9263: f64, t9268: f64, t5: f64, t2235: f64, t2240: f64, t2241: f64, t2307: f64, t605: f64, t645: f64, t86: f64, t9226: f64, t9228: f64, t9231: f64, t9239: f64, t9240: f64, t9243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9289, t9293, t9296, t9300, t9301, t9305, t9308, t9311) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1299(t9287, t9288, t2267, t607, t2250, t43, t9258, t53, t54, t2274, t55, t2585);
        let t9312 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1300(t2262, t2268, t2271, t39, t44, t51, t615, t618, t9277, t9289, t9293, t9296, t9301, t9305, t9308, t9311);
        let (t9313, t9321, t9330, t9338) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1301(t33, t9312, t2769, t73, t2291, t607, t3241, t76, t2298, t2250, t634, t638, t9258, t9288);
        let (t9339, t9342) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1302(t72, t9338, t2245, t2252, t2255, t2284, t2304, t609, t629, t642, t66, t80, t9247, t9248, t9251, t9260, t9263, t9268, t9313);
        let t9346 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1303(t5, t2235, t2240, t2241, t2307, t605, t645, t86, t9226, t9228, t9231, t9239, t9240, t9243, t9342);
    (t9300, t9301, t9305, t9308, t9311, t9312, t9313, t9321, t9330, t9339, t9342, t9346)
}
