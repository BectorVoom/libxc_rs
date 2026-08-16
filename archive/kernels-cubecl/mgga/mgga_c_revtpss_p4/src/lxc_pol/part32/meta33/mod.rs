//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk217;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk218;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk219;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk220;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk221;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk222;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk223;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta33<F: Float>(t635: F, t81: F, t606: F, t633: F, t77: F, t608: F, t628: F, t71: F, t85: F, t5: F, t599: F, t603: F, t91: F, t117: F, t116: F, t94: F, t112: F, t625: F, t111: F, t43: F, t605: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t637 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk217::<F>(t635, t81);
        let t640 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk218::<F>(t606, t633, t637);
        let (t641, t644) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk219::<F>(t640, t77, t608, t628, t71, t85);
        let t648 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk220::<F>(t5, t599, t603, t644, t91);
        let t649 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk221::<F>(t117, t648);
        let t651 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk222::<F>(t116, t94);
        let (t653, t654, t655) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk223::<F>(t112, t625, t111);
        let (t656, t658) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk224::<F>(t43, t605, tau0);
    (t637, t640, t641, t644, t648, t649, t651, t653, t654, t655, t656, t658)
}
