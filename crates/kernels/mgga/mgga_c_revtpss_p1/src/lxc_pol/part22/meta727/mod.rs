//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta727 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta727<F: Float>(t39875: F, t745: F, t9417: F, t760: F, t2596: F, t39871: F, t10587: F, t2626: F, t2523: F, t9425: F, t2389: F, t37: F) -> (F, F, F, F, F, F, F) {
        let (t40192, t40194, t40196, t40198, t40203, t40205, t40207) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2783::<F>(t39875, t745, t9417, t760, t2596, t39871, t10587, t2626, t2523, t9425, t2389, t37);
    (t40192, t40194, t40196, t40198, t40203, t40205, t40207)
}
