//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta108 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk620;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk621;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk622;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk623;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk624;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta108<F: Float>(t3303: F, t3603: F, t1243: F, t3140: F, t460: F, t471: F, t498: F, t530: F, t566: F, t525: F, t527: F, t2608: F, t520: F, t512: F, t19: F, t27: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3769 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk620::<F>(t3303, t3603);
        let t3781 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk621::<F>(t1243, t3140);
        let t3782 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk622::<F>(t3781, t460);
        let t3783 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk623::<F>(t3303, t471);
        let (t3800, t3801, t3828, t3833, t3841, t3853) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk624::<F>(t498, t530, t566, t525, t527, t2608, t520);
        let (t3854, t3857) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk625::<F>(t3853, t512, t19, t27);
    (t3769, t3781, t3782, t3783, t3800, t3801, t3828, t3833, t3841, t3853, t3854, t3857)
}
