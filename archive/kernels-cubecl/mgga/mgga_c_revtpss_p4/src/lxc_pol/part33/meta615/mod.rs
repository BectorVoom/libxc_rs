//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2048;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta615<F: Float>(t25878: F, t98067: F, t97732: F, t27840: F, t689: F, t94674: F, t94669: F, t26069: F, t97922: F, t28011: F, t686: F, t72: F, t7284: F, t7289: F, t10073: F, t25937: F, t7282: F, t7910: F, t25899: F, t97899: F, t25953: F, t27899: F, t25981: F, t5677: F, t820: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98069, t98071, t98078, t98081, t98084, t98087) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2048::<F>(t25878, t98067, t97732, t27840, t689, t94674, t94669, t26069, t97922, t28011, t686, t72);
        let (t98089, t98091, t98099, t98101, t98104, t98108) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2049::<F>(t7284, t98087, t7289, t10073, t25937, t7282, t7910, t25899, t97899, t25953, t27899, t25981, t5677, t820, t844);
    (t98069, t98071, t98078, t98081, t98084, t98089, t98091, t98099, t98101, t98104, t98108)
}
