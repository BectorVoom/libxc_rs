//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta108 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk620;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk621;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk622;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk623;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk624;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta108(t3303: f64, t3603: f64, t1243: f64, t3140: f64, t460: f64, t471: f64, t498: f64, t530: f64, t566: f64, t525: f64, t527: f64, t2608: f64, t520: f64, t512: f64, t19: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3769 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk620(t3303, t3603);
        let t3781 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk621(t1243, t3140);
        let t3782 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk622(t3781, t460);
        let t3783 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk623(t3303, t471);
        let (t3800, t3801, t3828, t3833, t3841, t3853) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk624(t498, t530, t566, t525, t527, t2608, t520);
        let (t3854, t3857) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk625(t3853, t512, t19, t27);
    (t3769, t3781, t3782, t3783, t3800, t3801, t3828, t3833, t3841, t3853, t3854, t3857)
}
