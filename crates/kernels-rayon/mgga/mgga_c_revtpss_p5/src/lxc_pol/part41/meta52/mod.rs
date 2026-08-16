//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk316;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk317;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta52(t1015: f64, t606: f64, t1012: f64, t225: f64, t989: f64, t366: f64, t994: f64, t373: f64, t999: f64, t372: f64, t371: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1016, t1017, t1020, t1021, t1024) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk316(t1015, t606, t1012, t225, t989, t366, t994);
        let t1025 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk317(t1024, t366);
        let (t1026, t1028, t1031, t1032) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk318(t373, t999, t372, t371, t196);
    (t1016, t1017, t1020, t1021, t1024, t1025, t1026, t1028, t1031, t1032)
}
