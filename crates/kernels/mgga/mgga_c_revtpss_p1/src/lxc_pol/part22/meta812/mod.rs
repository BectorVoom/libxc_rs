//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta812 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2916;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta812<F: Float>(t4066: F, t4086: F, t786: F, t10022: F, t2453: F, t281: F, t4003: F, t46507: F, t268: F, t39644: F, t546: F, t555: F, t8779: F, t1432: F, t4107: F, t9288: F, t10107: F, t3964: F, t9285: F, t39494: F, t4096: F, t40270: F, t4089: F, t138: F, t2438: F, t4131: F, t9674: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47423, t47429, t47432, t47442) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2916::<F>(t4066, t4086, t786, t10022, t2453, t281, t4003, t46507, t268, t39644, t546, t555, t8779);
        let (t47444, t47450, t47454, t47455, t47466) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2917::<F>(t1432, t4107, t9288, t10107, t3964, t9285, t39494, t4096, t40270, t4089, t138, t2438, t4131, t9674);
    (t47423, t47429, t47432, t47442, t47444, t47450, t47454, t47455, t47466)
}
