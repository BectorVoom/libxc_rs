//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta88 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk612;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk613;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk614;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk615;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk616;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta88<F: Float>(t2229: F, t19: F, t601: F, t604: F, t84: F, t85: F, t24: F, t42: F, t54: F, t240: F, t59: F, t40: F, t632: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2230, t2232, t2235, t2239) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk612::<F>(t2229, t19, t601, t604, t84, t85);
        let t2240 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk613::<F>(t2239, t24);
        let t2267 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk614::<F>(t42);
        let t2274 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk615::<F>(t54);
        let t2281 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk616::<F>(t240, t59);
        let (t2282, t2289, t2291) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk617::<F>(t2281, t40, t632, t73);
    (t2230, t2232, t2235, t2239, t2240, t2267, t2274, t2281, t2282, t2289, t2291)
}
