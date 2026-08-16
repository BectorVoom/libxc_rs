//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1910;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta531(t5: f64, t28115: f64, t28157: f64, t117: f64, t7239: f64, t7898: f64, t197: f64, t530: f64, t2013: f64, t5627: f64, t8996: f64, t1310: f64, t1453: f64, t28050: f64, t28053: f64, t28058: f64, t28060: f64, t28062: f64, t28065: f64, t28069: f64, t4248: f64, t508: f64, t649: f64, t651: f64, t7007: f64, t7725: f64, t7883: f64, t7894: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t28159, t28160, t28165, t28166, t28167) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1910(t5, t28115, t28157, t117, t7239, t7898, t197, t530, t2013);
        let (t28168, t28171) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1911(t5627, t8996, t28167, t1310, t1453, t28050, t28053, t28058, t28060, t28062, t28065, t28069, t28160, t28165, t4248, t508, t649, t651, t7007, t7725, t7883, t7894);
    (t28159, t28160, t28166, t28167, t28168, t28171)
}
