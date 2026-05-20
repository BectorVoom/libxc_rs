//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1812;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta341<F: Float>(t287: F, t2922: F, t275: F, t11132: F, t2912: F, t698: F, t240: F, t624: F, t281: F, t283: F, t2909: F, t3252: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11298, t11299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1812::<F>(t287, t2922, t275);
        let (t11304, t11326, t11334, t11335, t11337, t11338, t11339, t11341) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1813::<F>(t11132, t2912, t698, t240, t624, t281, t283, t2909, t3252);
    (t11298, t11299, t11304, t11326, t11334, t11335, t11337, t11338, t11339, t11341)
}
