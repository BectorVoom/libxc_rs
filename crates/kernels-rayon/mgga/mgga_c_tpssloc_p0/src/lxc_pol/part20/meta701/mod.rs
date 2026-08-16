//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2669;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta701(t39540: f64, t2221: f64, t5168: f64, t39571: f64, t39581: f64, t2225: f64, t5154: f64, t9892: f64, t39601: f64, t39605: f64, t39607: f64, t39609: f64, t39549: f64, t39563: f64, t39570: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54427, t54429, t54430, t54431, t54433, t54435, t54436, t54437, t54438, t54439) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2669(t39540, t2221, t5168, t39571, t39581, t2225, t5154, t9892, t39601, t39605, t39607, t39609);
        let t54440 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2670(t39549, t39563, t39570, t39585, t39590, t39593, t39595, t54427, t54429, t54430, t54431, t54433, t54435, t54436, t54437, t54438, t54439);
    (t54427, t54429, t54430, t54431, t54433, t54435, t54436, t54437, t54438, t54439, t54440)
}
