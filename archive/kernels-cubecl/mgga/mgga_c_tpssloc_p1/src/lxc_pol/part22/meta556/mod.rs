//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2057;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta556<F: Float>(t207: F, t40419: F, t9538: F, t41083: F, t789: F, t154: F, t1891: F, t205: F, t792: F, t9558: F, t40394: F, t40399: F, t786: F, t9580: F, t2578: F, t2566: F, t2570: F, t2588: F, t40341: F, t215: F, t39933: F, t40344: F, t795: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41155, t41156, t41160, t41161, t41170, t41185) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2057::<F>(t207, t40419, t9538, t41083, t789, t154, t1891, t205, t792, t9558, t40394, t40399);
        let (t41189, t41190, t41196, t41200, t41209, t41212) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2058::<F>(t786, t9580, t2578, t2566, t2570, t2588, t40341, t207, t215, t39933, t40344, t795);
    (t41155, t41156, t41160, t41161, t41170, t41185, t41189, t41190, t41196, t41200, t41209, t41212)
}
