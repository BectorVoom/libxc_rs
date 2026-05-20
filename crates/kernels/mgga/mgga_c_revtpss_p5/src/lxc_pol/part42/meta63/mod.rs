//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta63 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk378;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk379;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk380;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk381;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk382;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk383;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta63<F: Float>(t1230: F, t480: F, t1209: F, t225: F, t1214: F, t482: F, t372: F, t371: F, t1032: F, t460: F, t472: F, t474: F, t1038: F, t479: F, t1128: F, t1153: F, t1193: F, t1195: F, t1200: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1231, t1234) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk378::<F>(t1230, t480, t1209, t225);
        let t1235 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk379::<F>(t1234, t480);
        let (t1236, t1238) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk380::<F>(t1214, t482, t372, t371);
        let (t1241, t1242, t1243, t1244, t1246) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk381::<F>(t1032, t460, t472, t474, t1038, t479);
        let t1247 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk382::<F>(t1241, t1246);
        let t1248 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk383::<F>(t1128, t1153, t1193, t1195, t1200);
    (t1231, t1234, t1235, t1236, t1238, t1241, t1242, t1243, t1244, t1246, t1247, t1248)
}
