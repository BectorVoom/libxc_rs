//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta65 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk388;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk389;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk390;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk391;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta65<F: Float>(t1214: F, t482: F, t372: F, t371: F, t1032: F, t460: F, t472: F, t474: F, t1038: F, t479: F, t1128: F, t1153: F, t1193: F, t1195: F, t1200: F, t471: F, t73: F, t1042: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1236, t1238, t1241, t1242, t1243, t1244, t1246) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk388::<F>(t1214, t482, t372, t371, t1032, t460, t472, t474, t1038, t479);
        let t1247 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk389::<F>(t1241, t1246);
        let t1248 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk390::<F>(t1128, t1153, t1193, t1195, t1200);
        let t1250 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk391::<F>(t471, t73);
        let (t1251, t1252) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk392::<F>(t1248, t1250, t482, t1042);
    (t1236, t1238, t1241, t1242, t1243, t1244, t1246, t1247, t1248, t1250, t1251, t1252)
}
