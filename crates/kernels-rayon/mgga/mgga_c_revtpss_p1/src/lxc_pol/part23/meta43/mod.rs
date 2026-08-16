//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta43 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk315;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk316;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk317;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk318;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk319;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta43(t256: f64, t225: f64, t212: f64, t233: f64, t251: f64, t689: f64, t234: f64, t786: f64, t72: f64, t686: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t866, t867) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk315(t256);
        let t868 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk316(t225, t867);
        let t869 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk317(t212, t225);
        let (t870, t871, t873, t874) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk318(t233, t251, t869, t689, t234, t786);
        let t875 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk319(t251, t72);
        let (t878, t879) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk320(t686, t874, t875, t251, t822);
    (t866, t867, t868, t869, t870, t871, t873, t874, t875, t878, t879)
}
