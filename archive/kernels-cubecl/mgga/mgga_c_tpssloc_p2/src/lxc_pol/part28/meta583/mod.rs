//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1871;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta583<F: Float>(t252: F, t87230: F, t13230: F, t87052: F, t23168: F, t25321: F, t25284: F, t6579: F, t13388: F, t1888: F, t6646: F, t13385: F, t22996: F, t23185: F, t4283: F, t81914: F, t25300: F, t81591: F, t1484: F, t6552: F, t6637: F, t81658: F, t25303: F, t13456: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87531, t87533, t87535, t87538, t87541) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1871::<F>(t252, t87230, t13230, t87052, t23168, t25321, t25284, t6579, t13388, t1888, t6646, t13385, t22996);
        let (t87544, t87546, t87554, t87565, t87575) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1872::<F>(t23185, t4283, t81914, t25300, t81591, t1484, t6552, t6637, t81658, t25303, t6579, t13456, t1888, t6646);
    (t87531, t87533, t87535, t87538, t87541, t87544, t87546, t87554, t87565, t87575)
}
