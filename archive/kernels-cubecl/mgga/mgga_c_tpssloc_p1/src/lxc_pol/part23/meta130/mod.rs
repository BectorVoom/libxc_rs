//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk636;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta130<F: Float>(t109: F, t5488: F, t656: F, t2327: F, t4041: F, t5465: F, t64: F, t40: F, t510: F, t4100: F, t4102: F, t185: F, t5392: F, t2658: F, t1484: F, t4310: F, t1462: F, t4205: F, t2433: F, t5398: F, t73: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5489, t5493) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk636::<F>(t109, t5488, t656, t2327, t4041, t5465, t64);
        let (t5494, t5497, t5498, t5499, t5501, t5502, t5506, t5512) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk637::<F>(t40, t510, t5493, t4100, t4102, t185, t5392, t2658, t1484, t4310, t1462, t4205, t2433, t5398, t73, zeta_threshold);
    (t5489, t5493, t5494, t5497, t5498, t5499, t5501, t5502, t5506, t5512)
}
