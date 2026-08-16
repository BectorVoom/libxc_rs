//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk773;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk774;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta158<F: Float>(t33: F, t9312: F, t2769: F, t73: F, t2291: F, t607: F, t3241: F, t76: F, t2298: F, t2250: F, t634: F, t638: F, t9258: F, t9288: F, t72: F, t2245: F, t2252: F, t2255: F, t2284: F, t2304: F, t609: F, t629: F, t642: F, t66: F, t80: F, t9247: F, t9248: F, t9251: F, t9260: F, t9263: F, t9268: F, t5: F, t2235: F, t2240: F, t2241: F, t2307: F, t605: F, t645: F, t86: F, t9226: F, t9228: F, t9231: F, t9239: F, t9240: F, t9243: F) -> (F, F, F, F, F, F, F, F) {
        let (t9313, t9321, t9324, t9330, t9333, t9338) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk773::<F>(t33, t9312, t2769, t73, t2291, t607, t3241, t76, t2298, t2250, t634, t638, t9258, t9288);
        let (t9339, t9342) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk774::<F>(t72, t9338, t2245, t2252, t2255, t2284, t2304, t609, t629, t642, t66, t80, t9247, t9248, t9251, t9260, t9263, t9268, t9313);
        let t9346 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk775::<F>(t5, t2235, t2240, t2241, t2307, t605, t645, t86, t9226, t9228, t9231, t9239, t9240, t9243, t9342);
    (t9313, t9321, t9324, t9330, t9333, t9339, t9342, t9346)
}
