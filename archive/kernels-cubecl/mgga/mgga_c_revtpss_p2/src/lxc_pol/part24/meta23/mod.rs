//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta23 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk176;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk177;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk178;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk179;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk180;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk181;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk182;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk183;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk184;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta23<F: Float>(t225: F, t460: F, t355: F, t424: F, t452: F, t454: F, sigma2: F, t51: F, t52: F, rho1: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t467 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk176::<F>(t225, t460);
        let t471 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk177::<F>(t225, t355, t424, t452, t454);
        let (t472, t473) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk178::<F>(t471);
        let t474 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk179::<F>(sigma2);
        let t475 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk180::<F>(t473, t474);
        let t476 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk181::<F>(t51);
        let (t477, t479) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk182::<F>(t476, t52, rho1);
        let t480 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk183::<F>(t475, t479);
        let t481 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk184::<F>(t467, t480);
        let t482 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk185::<F>(t414);
    (t467, t471, t472, t473, t474, t475, t476, t477, t479, t480, t481, t482)
}
