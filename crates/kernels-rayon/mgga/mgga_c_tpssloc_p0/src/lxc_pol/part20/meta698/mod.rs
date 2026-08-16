//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta698 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2664;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta698(t157: f64, t54349: f64, t54372: f64, t17: f64, t184: f64, t39324: f64, t39327: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t54313: f64, t54315: f64, t54317: f64, t54318: f64, t54319: f64, t54320: f64, t54321: f64, t54322: f64, t54324: f64, t54326: f64, t39365: f64, t15908: f64, t9885: f64, t9888: f64, t39374: f64, t39387: f64, t15968: f64, t172: f64, t763: f64, t5154: f64, t9713: f64, t39360: f64, t39364: f64, t39373: f64, t39384: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54374, t54376, t54377) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2664(t157, t54349, t54372, t17, t184, t39324, t39327, t39338, t39346, t39349, t39356, t54313, t54315, t54317, t54318, t54319, t54320, t54321, t54322, t54324, t54326);
        let (t54379, t54381, t54383, t54384, t54385, t54388, t54390, t54391) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2665(t39365, t15908, t9885, t9888, t39374, t39387, t15968, t172, t763, t5154, t9713, t39360, t39364, t39373, t39384, t39393, t39397, t39400, t39408, t39411);
    (t54374, t54376, t54377, t54379, t54381, t54383, t54384, t54385, t54388, t54390, t54391)
}
