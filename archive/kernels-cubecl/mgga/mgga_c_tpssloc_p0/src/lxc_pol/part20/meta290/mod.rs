//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1496;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1497;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1498;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta290<F: Float>(t10481: F, t360: F, t1021: F, t248: F, t1030: F, t3036: F, t1015: F, t3033: F, t3041: F, t3101: F, t3039: F, t3108: F, t3113: F, t3128: F, t3121: F, t1020: F, t2250: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10884, t10886, t10889, t10890, t10891) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1496::<F>(t10481, t360, t1021, t248, t1030, t3036, t1015, t3033);
        let (t10895, t10896, t10898) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1497::<F>(t248, t3041, t3101, t3039, t3108, t3113);
        let (t10903, t10904) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1498::<F>(t10889, t3128, t3033);
        let (t10908, t10909, t10913) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1499::<F>(t248, t3101, t3121, t1020, t2250, t607);
    (t10884, t10886, t10890, t10891, t10895, t10896, t10898, t10903, t10904, t10908, t10909, t10913)
}
