//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta810 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2652;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2653;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2654;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta810<F: Float>(t2435: F, t6093: F, t6097: F, t6101: F, t2439: F, t6132: F, t6135: F, t19013: F, t698: F, t19016: F, t6138: F, t18960: F, t18963: F, t18966: F, t19077: F, t914: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t63453 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2652::<F>(t2435, t6093);
        let t63459 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2653::<F>(t2435, t6097);
        let t63464 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2654::<F>(t2435, t6101);
        let (t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t63610) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2655::<F>(t2439, t6132, t6135, t19013, t698, t19016, t6138, t18960, t18963, t18966, t19077, t914);
    (t63453, t63459, t63464, t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t63610)
}
