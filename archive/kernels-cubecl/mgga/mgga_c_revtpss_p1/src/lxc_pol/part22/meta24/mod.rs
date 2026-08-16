//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta24 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk184;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk185;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk186;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk187;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk188;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk189;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk190;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk191;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk192;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta24<F: Float>(t51: F, t52: F, rho1: F, t475: F, t467: F, t414: F, t371: F, t372: F, t461: F, t464: F, t225: F, t473: F, t460: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t476, t479) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk184::<F>(t51, t52, rho1);
        let t480 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk185::<F>(t475, t479);
        let t481 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk186::<F>(t467, t480);
        let t482 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk187::<F>(t414);
        let t484 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk188::<F>(t371, t372, t482);
        let t487 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk189::<F>(t461, t464, t481, t484);
        let t488 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk190::<F>(t225, t487);
        let t489 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk191::<F>(t225, t473);
        let t490 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk192::<F>(t487, t489);
        let (t493, t494) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk193::<F>(t460, t490);
    (t476, t479, t480, t481, t482, t484, t487, t488, t489, t490, t493, t494)
}
