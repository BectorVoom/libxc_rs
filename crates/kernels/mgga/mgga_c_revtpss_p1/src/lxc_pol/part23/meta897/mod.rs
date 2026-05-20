//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta897 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta897<F: Float>(t50084: F, t61239: F, t50092: F, t50094: F, t23221: F, t2398: F, t61247: F, t61282: F, t61289: F, t50852: F, t50856: F, t61294: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t77002, t77003, t77004, t77005, t77007, t77008, t77009, t77010, t77011, t77012, t77013) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2856::<F>(t50084, t61239, t50092, t50094, t23221, t2398, t61247, t61282, t61289, t50852, t50856, t61294);
    (t77002, t77003, t77004, t77005, t77007, t77008, t77009, t77010, t77011, t77012, t77013)
}
