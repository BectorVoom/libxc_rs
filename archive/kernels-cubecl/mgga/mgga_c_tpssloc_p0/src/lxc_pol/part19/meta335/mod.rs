//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta335<F: Float>(t39537: F, t761: F, t2531: F, t9494: F, t39344: F, t39362: F, t2427: F, t9868: F, t2749: F, t2751: F, t12908: F, t9682: F) -> (F, F, F, F, F, F, F, F) {
        let (t40760, t40762, t40764, t40766, t40768, t40769, t40772, t40777) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1199::<F>(t39537, t761, t2531, t9494, t39344, t39362, t2427, t9868, t2749, t2751, t12908, t9682);
    (t40760, t40762, t40764, t40766, t40768, t40769, t40772, t40777)
}
