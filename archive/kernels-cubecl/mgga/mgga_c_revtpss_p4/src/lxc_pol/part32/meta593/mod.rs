//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta593<F: Float>(t99033: F, t99041: F, t99066: F, t99069: F, t99073: F, t99077: F, t99085: F, t99099: F, t99102: F, t136: F, t2457: F, t8015: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t103296, t103301, t103315, t103316, t103318, t103320, t103324, t103336, t103337, t103363) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1925::<F>(t99033, t99041, t99066, t99069, t99073, t99077, t99085, t99099, t99102, t136, t2457, t8015);
    (t103296, t103301, t103315, t103316, t103318, t103320, t103324, t103336, t103337, t103363)
}
