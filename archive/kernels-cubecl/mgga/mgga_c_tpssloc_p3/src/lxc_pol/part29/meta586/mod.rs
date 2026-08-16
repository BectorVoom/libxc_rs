//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2008;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta586<F: Float>(t1354: F, t80991: F, t1336: F, t22759: F, t835: F, t3795: F, t22765: F, t3853: F, t22704: F, t22898: F, t80798: F, t12248: F, t6604: F, t22720: F, t6883: F, t22716: F, t6983: F, t22742: F, t6914: F, t22748: F, t80727: F, t22723: F, t268: F, t534: F, t22706: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t80992, t80998, t81007, t81022, t81027) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2008::<F>(t1354, t80991, t1336, t22759, t835, t3795, t22765, t3853, t22704, t22898, t80798, t12248, t6604);
        let (t81037, t81039, t81041, t81043, t81046, t81047) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2009::<F>(t22720, t6883, t22716, t6983, t22742, t6914, t22748, t80727, t22723, t268, t534, t22706);
    (t80992, t80998, t81007, t81022, t81027, t81037, t81039, t81041, t81043, t81046, t81047)
}
