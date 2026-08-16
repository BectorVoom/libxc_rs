//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta781 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta781<F: Float>(t39611: F, t39636: F, t57211: F, t17: F, t184: F, t74011: F, t54451: F, t20396: F, t750: F, t39845: F, t39615: F, t39620: F, t39655: F, t39658: F, t39844: F, t54439: F, t54447: F, t54453: F, t54457: F) -> (F, F, F, F, F, F, F, F) {
        let (t74489, t74490, t74491, t74493, t74494, t74496, t74497, t74498) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2671::<F>(t39611, t39636, t57211, t17, t184, t74011, t54451, t20396, t750, t39845, t39615, t39620, t39655, t39658, t39844, t54439, t54447, t54453, t54457);
    (t74489, t74490, t74491, t74493, t74494, t74496, t74497, t74498)
}
