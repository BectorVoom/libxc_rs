//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta266 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk988;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk989;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk990;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk991;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk992;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta266<F: Float>(t730: F, t9446: F, t2596: F, t675: F, t215: F, t723: F, t2553: F, t738: F, t2491: F, t177: F, t9417: F, t2495: F, t9368: F, t2531: F, t2536: F, t2539: F, t2549: F, t2557: F, t2591: F, t2598: F, t2601: F, t2605: F, t268: F, t724: F, t731: F, t746: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t9433: F, t9435: F, t745: F, t9385: F, t2514: F, t744: F, t2576: F, t2582: F, t2584: F, t700: F, t2519: F, t2577: F, t2581: F, t2585: F, t2565: F, t702: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk988::<F>(t730, t9446, t2596, t675, t215, t723, t2553, t738, t2491, t177, t9417, t2495, t9368);
        let t9484 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk989::<F>(t2531, t2536, t2539, t2549, t2557, t2591, t2598, t2601, t2605, t268, t675, t724, t731, t746, t9278, t9308, t9316, t9329, t9333, t9433, t9435, t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481);
        let (t9485, t9488, t9501, t9508, t9514) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk990::<F>(t745, t9385, t9368, t2514, t746, t2495, t744, t2576, t2582, t2584, t700);
        let t9517 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk991::<F>(t2519, t2577, t268);
        let t9521 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk992::<F>(t2581, t675, t2585, t268);
        let t9524 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk993::<F>(t2565, t2576, t702);
    (t9484, t9485, t9488, t9501, t9508, t9514, t9517, t9521, t9524)
}
