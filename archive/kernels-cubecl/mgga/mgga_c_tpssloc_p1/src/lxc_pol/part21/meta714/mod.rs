//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta714 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2552;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2553;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta714<F: Float>(t14134: F, t3117: F, t10863: F, t4571: F, t13969: F, t14102: F, t3039: F, t10876: F, t13990: F, t3048: F, t14137: F, t10952: F, t13970: F, t14098: F, t10224: F, t4343: F, t973: F, t3130: F, t4595: F, t49850: F, t10402: F, t14618: F, t14608: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49873, t49877, t49884, t49887, t49889, t49892, t49894) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2552::<F>(t14134, t3117, t10863, t4571, t13969, t14102, t3039, t10876, t13990, t3048, t14137, t10952, t13970);
        let (t49897, t49906, t49922, t49929, t49934) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2553::<F>(t13969, t14098, t3039, t10224, t4343, t973, t3130, t4595, t49850, t10402, t14618, t14608);
    (t49873, t49877, t49884, t49887, t49889, t49892, t49894, t49897, t49906, t49922, t49929, t49934)
}
