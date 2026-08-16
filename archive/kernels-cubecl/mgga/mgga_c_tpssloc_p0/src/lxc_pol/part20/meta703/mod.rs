//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta703<F: Float>(t39845: F, t2221: F, t5166: F, t2223: F, t1788: F, t9216: F, t9218: F, t39851: F, t39855: F, t39857: F, t5154: F, t9494: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t54455, t54457, t54459, t54461, t54463, t54464, t54465, t54466, t54467) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2673::<F>(t39845, t2221, t5166, t2223, t1788, t9216, t9218, t39851, t39855, t39857, t5154, t9494);
    (t54455, t54457, t54459, t54461, t54463, t54464, t54465, t54466, t54467)
}
