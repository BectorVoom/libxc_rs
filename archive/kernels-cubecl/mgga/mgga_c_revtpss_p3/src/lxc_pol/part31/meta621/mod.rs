//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta621<F: Float>(t7058: F, t99321: F, t7759: F, t822: F, t25310: F, t27279: F, t27186: F, t93321: F, t93374: F, t122: F, t72: F, t2466: F) -> (F, F, F, F, F, F, F) {
        let (t99323, t99334, t99342, t99344, t99346, t99348, t99349) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2071::<F>(t7058, t99321, t7759, t822, t25310, t27279, t27186, t93321, t93374, t122, t72, t2466);
    (t99323, t99334, t99342, t99344, t99346, t99348, t99349)
}
