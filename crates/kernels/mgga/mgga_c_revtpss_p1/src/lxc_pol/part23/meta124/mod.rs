//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk810;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk811;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk812;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk813;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk814;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta124<F: Float>(t3356: F, t406: F, t281: F, t2902: F, t414: F, t1146: F, t698: F, t1224: F, t240: F, t1129: F, t408: F, t421: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3394, t3402, t3407) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk810::<F>(t3356, t406);
        let (t3413, t3414, t3415, t3417) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk811::<F>(t281, t2902, t414, t1146, t698, t1224, t240);
        let t3431 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk812::<F>(t1129);
        let t3432 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk813::<F>(t3431);
        let t3433 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk814::<F>(t3432, t408);
        let (t3434, t3435) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk815::<F>(t421);
    (t3394, t3402, t3407, t3413, t3414, t3415, t3417, t3431, t3432, t3433, t3434, t3435)
}
