//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta41 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk292;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta41<F: Float>(t287: F, t275: F, t276: F, t902: F, t273: F, t240: F, t696: F, t281: F, t283: F, t346: F, t290: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t913, t914, t915, t916, t921, t923, t926, t928, t929, t930) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk292::<F>(t287, t275, t276, t902, t273, t240, t696, t281, t283, t346);
        let t935 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk293::<F>(t290);
    (t913, t914, t915, t916, t921, t923, t926, t928, t929, t930, t935)
}
