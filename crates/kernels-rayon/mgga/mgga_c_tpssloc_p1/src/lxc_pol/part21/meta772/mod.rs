//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta772 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2673;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta772(t39365: f64, t19681: f64, t2371: f64, t54380: f64, t54382: f64, t39374: f64, t39387: f64, t20067: f64, t3719: f64, t3918: f64, t39360: f64, t39364: f64, t39373: f64, t39384: f64, t54387: f64, t54389: f64, t19575: f64, t592: f64, t15904: f64, t16486: f64, t16497: f64, t1845: f64, t193: f64, t19603: f64, t33159: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t5126: f64, t5160: f64, t5161: f64, t5308: f64, t531: f64, t55224: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56167, t56169, t56170, t56171, t56172, t56173, t56174) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2673(t39365, t19681, t2371, t54380, t54382, t39374, t39387, t20067, t3719, t3918, t39360, t39364, t39373, t39384);
        let (t56178, t56179, t56186, t56192) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2674(t54387, t54389, t19575, t592, t15904, t16486, t16497, t1845, t193, t19603, t33159, t39393, t39397, t39400, t39408, t39411, t5126, t5160, t5161, t5308, t531, t55224);
    (t56167, t56169, t56170, t56171, t56172, t56173, t56174, t56178, t56179, t56186, t56192)
}
