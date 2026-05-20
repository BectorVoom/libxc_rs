//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta723<F: Float>(t2523: F, t9318: F, t2596: F, t746: F, t9385: F, t760: F, t186: F, t2698: F, t685: F, t755: F, t2491: F, t2495: F, t39871: F) -> (F, F, F, F, F, F) {
        let (t40094, t40097, t40099, t40101, t40103, t40113) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2779::<F>(t2523, t9318, t2596, t746, t9385, t760, t186, t2698, t685, t755, t2491, t2495, t39871);
    (t40094, t40097, t40099, t40101, t40103, t40113)
}
