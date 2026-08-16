//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk310;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk311;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk312;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk313;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk314;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk315;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta51(t225: f64, t385: f64, t902: f64, t908: f64, t344: f64, t614: f64, t139: f64, t221: f64, t346: f64, t345: f64, t220: f64, t44: f64, t124: f64, t65: f64, t270: f64, t271: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t996 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk310(t225, t385);
        let (t997, t999) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk311(t902, t908);
        let (t1000, t1003, t1007, t1009, t1010) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk312(t996, t999, t344, t614, t139, t221, t346, t345, t220);
        let t1011 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk313(t1010, t44);
        let t1012 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk314(t124, t65);
        let t1014 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk315(t270, t271);
        let t1015 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk316(t1014, t905);
    (t996, t997, t999, t1000, t1003, t1007, t1009, t1010, t1011, t1012, t1014, t1015)
}
