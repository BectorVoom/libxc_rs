//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta60 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk365;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk366;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk367;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta60<F: Float>(t1149: F, t1150: F, t1131: F, t1118: F, t1124: F, t431: F, t426: F, t1143: F, t1135: F, t1140: F, t1147: F, t434: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1151, t1153, t1154, t1156, t1159, t1160) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk365::<F>(t1149, t1150, t1131, t1118, t1124, t431);
        let t1161 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk366::<F>(t1160, t426);
        let (t1163, t1166, t1168) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk367::<F>(t1118, t1143, t1124, t1135, t1140, t1147);
        let t1169 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk368::<F>(t434);
    (t1151, t1153, t1154, t1156, t1159, t1160, t1161, t1163, t1166, t1168, t1169)
}
