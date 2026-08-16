//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta112 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk751;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk752;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk753;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk754;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk755;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta112<F: Float>(t2289: F, t2244: F, t882: F, t123: F, t2250: F, t883: F, t2765: F, t2766: F, t2773: F, t291: F, t888: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t2775 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk751::<F>(t2289);
        let t2776 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk752::<F>(t2244, t2775);
        let (t2777, t2778) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk753::<F>(t2776, t882, t123);
        let t2780 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk754::<F>(t2250, t883);
        let (t2781, t2782) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk755::<F>(t2780, t882, t123);
        let (t2784, t2786, t2787) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk756::<F>(t2765, t2766, t2773, t2778, t2782, t291, t888, t892);
    (t2775, t2776, t2777, t2778, t2780, t2781, t2782, t2784, t2786, t2787)
}
