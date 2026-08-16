//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk729;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk730;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk731;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta129(t2258: f64, t905: f64, t904: f64, t128: f64, t2847: f64, t2848: f64, t2855: f64, t2860: f64, t291: f64, t910: f64, t914: f64, t936: f64, t287: f64, t913: f64, t275: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2862 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk729(t2258, t905);
        let (t2863, t2864) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk730(t2862, t904, t128);
        let (t2866, t2868, t2869, t2871, t2873, t2874) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk731(t2847, t2848, t2855, t2860, t2864, t291, t910, t914, t936, t287, t913, t275);
        let t2875 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk732(t934);
    (t2862, t2863, t2864, t2866, t2868, t2869, t2871, t2873, t2874, t2875)
}
