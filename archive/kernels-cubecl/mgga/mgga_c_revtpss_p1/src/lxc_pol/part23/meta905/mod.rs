//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta905 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2906;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2907;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2908;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2909;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2910;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta905<F: Float>(t22688: F, t41296: F, t606: F, t128: F, t41339: F, t22671: F, t2857: F, t904: F, t2852: F, t2850: F, t23470: F, t18908: F, t4186: F, t52037: F, t52346: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t77573, t77575) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2906::<F>(t22688, t41296, t606, t128, t41339);
        let (t77579, t77581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2907::<F>(t22671, t2857, t606, t128, t904);
        let (t77584, t77586) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2908::<F>(t22671, t2852, t606, t128, t2850);
        let (t77588, t77590) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2909::<F>(t23470, t606, t128, t2850);
        let (t77592, t77594) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2910::<F>(t18908, t4186, t128, t2850);
        let t77596 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2911::<F>(t52037, t52346, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
    (t77573, t77575, t77579, t77581, t77584, t77586, t77588, t77590, t77592, t77594, t77596)
}
