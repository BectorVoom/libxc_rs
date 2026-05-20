//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk884;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta227<F: Float>(t2880: F, t6113: F, t2884: F, t4571: F, t6094: F, t6098: F, t6102: F, t916: F, t2897: F, t923: F, t2908: F, t6092: F, t141: F, t6096: F, t930: F, t6100: F, t2892: F, t2905: F, t4620: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6114, t6120, t6121, t6127, t6129, t6132) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk884::<F>(t2880, t6113, t2884, t4571, t6094, t6098, t6102, t916, t2897, t923, t2908, t6092);
        let (t6133, t6135, t6136, t6138, t6139, t6141) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk885::<F>(t141, t6132, t6096, t930, t6100, t2892, t2905, t4571, t4620, t6094, t6098, t6102, t6114, t6121, t6127, t6129);
    (t6114, t6120, t6121, t6127, t6129, t6132, t6133, t6135, t6136, t6138, t6139, t6141)
}
