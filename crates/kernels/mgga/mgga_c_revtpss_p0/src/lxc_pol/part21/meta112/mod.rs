//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta112 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk729;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk730;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk731;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk732;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk733;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta112<F: Float>(t2703: F, t802: F, t124: F, t2430: F, t800: F, t234: F, t2453: F, t595: F, t65: F, t235: F, t826: F, t232: F, t821: F, t239: F, t820: F, t836: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2704, t2706, t2707, t2710) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk729::<F>(t2703, t802, t124, t2430, t800, t234, t2453);
        let (t2712, t2713) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk730::<F>(t595, t65, t235);
        let (t2716, t2718) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk731::<F>(t2710, t2713, t826, t232, t821);
        let t2719 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk732::<F>(t235, t2718);
        let (t2721, t2722) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk733::<F>(t239, t2719, t820, t836);
        let t2723 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk734::<F>(t231);
    (t2704, t2706, t2707, t2710, t2712, t2713, t2716, t2718, t2719, t2721, t2722, t2723)
}
