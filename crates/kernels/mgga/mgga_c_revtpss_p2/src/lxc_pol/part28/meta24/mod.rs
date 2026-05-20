//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta24 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk168;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk169;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk170;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk171;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk172;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta24<F: Float>(t221: F, t462: F, t65: F, t225: F, t460: F, t355: F, t424: F, t452: F, t454: F, sigma2: F, t51: F, t52: F, rho1: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t464, t467, t471) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk168::<F>(t221, t462, t65, t225, t460, t355, t424, t452, t454);
        let (t472, t473) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk169::<F>(t471);
        let t474 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk170::<F>(sigma2);
        let (t475, t476, t479) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk171::<F>(t473, t474, t51, t52, rho1);
        let t480 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk172::<F>(t475, t479);
        let (t481, t482) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk173::<F>(t467, t480, t414);
    (t464, t467, t471, t472, t473, t474, t475, t476, t479, t480, t481, t482)
}
