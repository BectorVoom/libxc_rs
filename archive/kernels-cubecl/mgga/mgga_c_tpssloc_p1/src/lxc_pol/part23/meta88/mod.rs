//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta88 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk502;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk503;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk504;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta88<F: Float>(t2884: F, t302: F, t310: F, t2764: F, t320: F, t941: F, t315: F, t2822: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2885, t2886, t2887, t2888) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk502::<F>(t2884, t302, t310);
        let (t2892, t2903, t2904) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk503::<F>(t2764, t320, t941);
        let (t2905, t2912, t2919, t2928) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk504::<F>(t2904, t315, t2764, t2822, t941);
        let t2929 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk505::<F>(t2928);
    (t2885, t2886, t2887, t2888, t2892, t2903, t2904, t2905, t2912, t2919, t2928, t2929)
}
