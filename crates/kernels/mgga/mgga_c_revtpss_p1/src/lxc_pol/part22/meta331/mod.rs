//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta331<F: Float>(t2778: F, t9303: F, t871: F, t9292: F, t2760: F, t72: F, t686: F, t874: F, t251: F, t9646: F, t22: F, t780: F) -> (F, F, F, F, F, F) {
        let (t10969, t10971, t10972, t10974, t10981, t10982) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1787::<F>(t2778, t9303, t871, t9292, t2760, t72, t686, t874, t251, t9646, t22, t780);
    (t10969, t10971, t10972, t10974, t10981, t10982)
}
