//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1780;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1781;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1782;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1783;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta394<F: Float>(t13532: F, t2768: F, t123: F, t13559: F, t882: F, t13542: F, t13546: F, t10296: F, t10298: F, t10302: F, t13567: F, t13569: F, t13572: F, t13575: F, t1540: F, t2394: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13577, t13578) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1780::<F>(t13532, t2768, t123);
        let (t13580, t13581) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1781::<F>(t13559, t882, t123);
        let (t13583, t13584) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1782::<F>(t13542, t882, t123);
        let (t13586, t13587) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1783::<F>(t13546, t882, t123);
        let (t13592, t13598) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1784::<F>(t10296, t10298, t10302, t13567, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t1540, t2394);
    (t13577, t13578, t13580, t13581, t13583, t13584, t13586, t13587, t13592, t13598)
}
