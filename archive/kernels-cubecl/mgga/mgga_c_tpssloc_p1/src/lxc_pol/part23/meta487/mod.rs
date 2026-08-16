//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta487<F: Float>(t54325: F, t56168: F, t54380: F, t54382: F, t20067: F, t20077: F, t39356: F, t39360: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t5126: F, t6330: F) -> (F, F, F, F, F) {
        let (t79896, t79897, t79898, t79899, t79903) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1493::<F>(t54325, t56168, t54380, t54382, t20067, t20077, t39356, t39360, t39364, t39373, t39384, t39393, t39397, t39400, t39408, t5126, t6330);
    (t79896, t79897, t79898, t79899, t79903)
}
