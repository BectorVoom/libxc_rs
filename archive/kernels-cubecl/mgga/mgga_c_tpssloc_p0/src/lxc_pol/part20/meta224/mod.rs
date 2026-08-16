//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta224 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1299;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1300;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1301;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1302;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1303;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta224<F: Float>(t9287: F, t9288: F, t2267: F, t607: F, t2250: F, t43: F, t9258: F, t53: F, t54: F, t2274: F, t55: F, t2585: F, t2262: F, t2268: F, t2271: F, t39: F, t44: F, t51: F, t615: F, t618: F, t9277: F, t33: F, t2769: F, t73: F, t2291: F, t3241: F, t76: F, t2298: F, t634: F, t638: F, t72: F, t2245: F, t2252: F, t2255: F, t2284: F, t2304: F, t609: F, t629: F, t642: F, t66: F, t80: F, t9247: F, t9248: F, t9251: F, t9260: F, t9263: F, t9268: F, t5: F, t2235: F, t2240: F, t2241: F, t2307: F, t605: F, t645: F, t86: F, t9226: F, t9228: F, t9231: F, t9239: F, t9240: F, t9243: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9289, t9293, t9296, t9300, t9301, t9305, t9308, t9311) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1299::<F>(t9287, t9288, t2267, t607, t2250, t43, t9258, t53, t54, t2274, t55, t2585);
        let t9312 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1300::<F>(t2262, t2268, t2271, t39, t44, t51, t615, t618, t9277, t9289, t9293, t9296, t9301, t9305, t9308, t9311);
        let (t9313, t9321, t9330, t9338) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1301::<F>(t33, t9312, t2769, t73, t2291, t607, t3241, t76, t2298, t2250, t634, t638, t9258, t9288);
        let (t9339, t9342) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1302::<F>(t72, t9338, t2245, t2252, t2255, t2284, t2304, t609, t629, t642, t66, t80, t9247, t9248, t9251, t9260, t9263, t9268, t9313);
        let t9346 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1303::<F>(t5, t2235, t2240, t2241, t2307, t605, t645, t86, t9226, t9228, t9231, t9239, t9240, t9243, t9342);
    (t9300, t9301, t9305, t9308, t9311, t9312, t9313, t9321, t9330, t9339, t9342, t9346)
}
