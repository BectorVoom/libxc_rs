//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1162;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1163;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta264(t2035: f64, t7235: f64, t2033: f64, t531: f64, t1353: f64, t1450: f64, t2014: f64, t2022: f64, t212: f64, t1358: f64, t689: f64, t2023: f64, t786: f64, t1364: f64, t533: f64, t7021: f64, t816: f64, t1941: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7236, t7237) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1162(t2035, t7235, t2033, t531);
        let (t7238, t7239, t7241, t7242) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1163(t1353, t1450, t7237, t2014, t2022, t212);
        let (t7243, t7245, t7246, t7248, t7251, t7252) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1164(t1358, t7242, t689, t2023, t786, t1364, t533, t7021, t816, t1941, t540);
    (t7236, t7237, t7238, t7239, t7241, t7242, t7243, t7245, t7246, t7248, t7251, t7252)
}
