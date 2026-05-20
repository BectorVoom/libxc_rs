//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk630;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta127<F: Float>(t2846: F, t273: F, t2881: F, t2889: F, t923: F, t240: F, t68: F, t281: F, t283: F, t698: F, t931: F, t1014: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2892, t2897, t2898, t2900, t2902, t2904, t2905, t2906) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk630::<F>(t2846, t273, t2881, t2889, t923, t240, t68, t281, t283, t698, t931);
        let t2908 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk631::<F>(t1014, t240);
    (t2892, t2897, t2898, t2900, t2902, t2904, t2905, t2906, t2908)
}
