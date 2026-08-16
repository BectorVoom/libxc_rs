//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk617;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk618;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk619;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk620;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk621;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk622;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk623;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta124(t2846: f64, t689: f64, t907: f64, t1065: f64, t159: f64, t631: f64, t2251: f64, t128: f64, t2297: f64, t904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2847, t2848) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk617(t2846, t689, t907);
        let t2850 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk618(t1065, t159);
        let (t2851, t2852) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk619(t631);
        let t2853 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk620(t2251, t2852);
        let (t2854, t2855) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk621(t2850, t2853, t128);
        let t2857 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk622(t2297);
        let t2858 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk623(t2251, t2857);
        let (t2859, t2860) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk624(t2858, t904, t128);
    (t2847, t2848, t2850, t2851, t2852, t2853, t2854, t2855, t2857, t2858, t2859, t2860)
}
