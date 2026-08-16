//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta772 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2673;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta772<F: Float>(t39365: F, t19681: F, t2371: F, t54380: F, t54382: F, t39374: F, t39387: F, t20067: F, t3719: F, t3918: F, t39360: F, t39364: F, t39373: F, t39384: F, t54387: F, t54389: F, t19575: F, t592: F, t15904: F, t16486: F, t16497: F, t1845: F, t193: F, t19603: F, t33159: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t5126: F, t5160: F, t5161: F, t5308: F, t531: F, t55224: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t56167, t56169, t56170, t56171, t56172, t56173, t56174) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2673::<F>(t39365, t19681, t2371, t54380, t54382, t39374, t39387, t20067, t3719, t3918, t39360, t39364, t39373, t39384);
        let (t56178, t56179, t56186, t56192) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2674::<F>(t54387, t54389, t19575, t592, t15904, t16486, t16497, t1845, t193, t19603, t33159, t39393, t39397, t39400, t39408, t39411, t5126, t5160, t5161, t5308, t531, t55224);
    (t56167, t56169, t56170, t56171, t56172, t56173, t56174, t56178, t56179, t56186, t56192)
}
