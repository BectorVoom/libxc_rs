//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta44 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk296;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk297;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk298;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk299;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk300;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta44(t256: f64, t225: f64, t212: f64, t233: f64, t251: f64, t689: f64, t234: f64, t786: f64, t72: f64, t686: f64, t822: f64, t837: f64, t860: f64, t213: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t866, t867) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk296(t256);
        let t868 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk297(t225, t867);
        let t869 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk298(t212, t225);
        let (t870, t871, t873, t874) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk299(t233, t251, t869, t689, t234, t786);
        let (t875, t878, t879) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk300(t251, t72, t686, t874, t822);
        let t886 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk301(t837, t879, t234, t860, t213, t820, t873, t878);
    (t866, t867, t868, t869, t870, t871, t873, t874, t875, t878, t879, t886)
}
