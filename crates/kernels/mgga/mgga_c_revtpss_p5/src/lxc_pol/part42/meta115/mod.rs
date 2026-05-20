//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk589;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta115<F: Float>(t910: F, t914: F, t287: F, t913: F, t275: F, t273: F, t276: F, t2846: F, t240: F, t68: F, t281: F, t283: F, t698: F, t931: F, t1014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2869, t2873, t2874, t2880, t2884, t2892, t2897, t2902, t2904) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk589::<F>(t910, t914, t287, t913, t275, t273, t276, t2846, t240, t68, t281, t283);
        let (t2905, t2906, t2908) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk590::<F>(t2904, t698, t931, t1014, t240);
    (t2869, t2873, t2874, t2880, t2884, t2892, t2897, t2902, t2904, t2905, t2906, t2908)
}
