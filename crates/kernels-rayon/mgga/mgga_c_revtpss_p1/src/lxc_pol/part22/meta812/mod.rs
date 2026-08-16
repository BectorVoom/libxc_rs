//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta812 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2916;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta812(t4066: f64, t4086: f64, t786: f64, t10022: f64, t2453: f64, t281: f64, t4003: f64, t46507: f64, t268: f64, t39644: f64, t546: f64, t555: f64, t8779: f64, t1432: f64, t4107: f64, t9288: f64, t10107: f64, t3964: f64, t9285: f64, t39494: f64, t4096: f64, t40270: f64, t4089: f64, t138: f64, t2438: f64, t4131: f64, t9674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47423, t47429, t47432, t47442) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2916(t4066, t4086, t786, t10022, t2453, t281, t4003, t46507, t268, t39644, t546, t555, t8779);
        let (t47444, t47450, t47454, t47455, t47466) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2917(t1432, t4107, t9288, t10107, t3964, t9285, t39494, t4096, t40270, t4089, t138, t2438, t4131, t9674);
    (t47423, t47429, t47432, t47442, t47444, t47450, t47454, t47455, t47466)
}
