//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk588;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk589;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta102<F: Float>(t2880: F, t2881: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t916: F, t273: F, t923: F, t240: F, t68: F, t281: F, t283: F, t698: F, t931: F, t1014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2882, t2889) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk588::<F>(t2880, t2881, t2846, t2848, t2855, t2860, t2864);
        let (t2890, t2892, t2897, t2898, t2900, t2902, t2904, t2905, t2906) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk589::<F>(t2889, t916, t2846, t273, t2881, t923, t240, t68, t281, t283, t698, t931);
        let t2908 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk590::<F>(t1014, t240);
    (t2882, t2889, t2890, t2892, t2897, t2898, t2900, t2902, t2904, t2905, t2906, t2908)
}
