//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk644;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk645;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk646;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta134(t3022: f64, t983: f64, t2986: f64, t2988: f64, t973: f64, t981: f64, t3006: f64, t964: f64, t3011: f64, t3014: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t341: f64, t988: f64, t993: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3024, t3026, t3028, t3030, t3032, t3034, t3036, t3037, t3042) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk644(t3022, t983, t2986, t2988, t973, t981, t3006, t964, t3011, t3014, t2846, t2848, t2855, t2860, t2864);
        let t3043 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk645(t3042, t341);
        let t3046 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk646(t988, t993);
        let t3047 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk647(t3046, t378);
    (t3024, t3026, t3028, t3030, t3032, t3034, t3036, t3037, t3042, t3043, t3046, t3047)
}
