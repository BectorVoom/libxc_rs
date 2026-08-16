//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta22 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk169;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk170;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk171;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk172;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta22<F: Float>(t408: F, t422: F, t406: F, t409: F, t412: F, t416: F, t300: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t424, t426) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk169::<F>(t408, t422, t406);
        let (t431, t434, t435) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk170::<F>(t406, t409, t412, t416);
        let t439 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk171::<F>(t406);
        let (t444, t447, t448) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk172::<F>(t406, t409, t412, t416);
        let (t452, t454, t456) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk173::<F>(t439, t448, t300, t424, t426, t435, t406);
    (t424, t426, t431, t434, t435, t439, t444, t447, t448, t452, t454, t456)
}
