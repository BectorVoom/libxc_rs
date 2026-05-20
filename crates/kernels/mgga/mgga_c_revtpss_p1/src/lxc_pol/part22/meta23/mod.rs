//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta23 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk175;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk176;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk177;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk178;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk179;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk180;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk181;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk182;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk183;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta23<F: Float>(t406: F, t456: F, t344: F, t56: F, t404: F, t221: F, t65: F, t225: F, t355: F, t424: F, t452: F, t454: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t458, t459) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk175::<F>(t406);
        let t460 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk176::<F>(t456, t459);
        let t461 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk177::<F>(t344, t56);
        let t462 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk178::<F>(t404);
        let (t464, t467) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk179::<F>(t221, t462, t65, t225, t460);
        let t471 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk180::<F>(t225, t355, t424, t452, t454);
        let (t472, t473) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk181::<F>(t471);
        let t474 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk182::<F>(sigma2);
        let t475 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk183::<F>(t473, t474);
    (t458, t459, t460, t461, t462, t464, t467, t471, t472, t473, t474, t475)
}
