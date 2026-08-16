//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2048;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta615(t25878: f64, t98067: f64, t97732: f64, t27840: f64, t689: f64, t94674: f64, t94669: f64, t26069: f64, t97922: f64, t28011: f64, t686: f64, t72: f64, t7284: f64, t7289: f64, t10073: f64, t25937: f64, t7282: f64, t7910: f64, t25899: f64, t97899: f64, t25953: f64, t27899: f64, t25981: f64, t5677: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98069, t98071, t98078, t98081, t98084, t98087) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2048(t25878, t98067, t97732, t27840, t689, t94674, t94669, t26069, t97922, t28011, t686, t72);
        let (t98089, t98091, t98099, t98101, t98104, t98108) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2049(t7284, t98087, t7289, t10073, t25937, t7282, t7910, t25899, t97899, t25953, t27899, t25981, t5677, t820, t844);
    (t98069, t98071, t98078, t98081, t98084, t98089, t98091, t98099, t98101, t98104, t98108)
}
