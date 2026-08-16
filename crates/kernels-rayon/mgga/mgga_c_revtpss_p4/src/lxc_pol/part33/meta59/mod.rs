//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta59 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk381;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk382;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk383;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta59(t1149: f64, t1150: f64, t1131: f64, t1118: f64, t1124: f64, t431: f64, t426: f64, t1143: f64, t1135: f64, t1140: f64, t1147: f64, t434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1151, t1153, t1154, t1156, t1159, t1160) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk381(t1149, t1150, t1131, t1118, t1124, t431);
        let t1161 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk382(t1160, t426);
        let (t1163, t1166, t1168) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk383(t1118, t1143, t1124, t1135, t1140, t1147);
        let t1169 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk384(t434);
    (t1151, t1153, t1154, t1156, t1159, t1160, t1161, t1163, t1166, t1168, t1169)
}
