//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta186 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1106;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta186<F: Float>(t5456: F, t89: F, t1458: F, t1774: F, t1453: F, t2331: F, t1444: F, t2341: F, t5396: F, t95: F, t1419: F, tau1: F, t1449: F, t2349: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5457, t5460, t5464, t5465, t5468, t5469, t5472, t5475) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1106::<F>(t5456, t89, t1458, t1774, t1453, t2331, t1444, t2341, t5396, t95, t1419, tau1);
        let (t5480, t5481, t5484) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1107::<F>(t1449, t2349, t5396);
    (t5457, t5460, t5464, t5465, t5468, t5469, t5472, t5475, t5480, t5481, t5484)
}
