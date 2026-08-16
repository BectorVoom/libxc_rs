//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta55 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk332;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk333;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk334;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk335;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk336;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk337;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta55(t1066: f64, t906: f64, t247: f64, t1003: f64, t1009: f64, t1011: f64, t1017: f64, t1021: f64, t1025: f64, t1028: f64, t1041: f64, t1047: f64, t1054: f64, t1060: f64, t1063: f64, t348: f64, t375: f64, t225: f64, t385: f64, t342: f64, t378: f64, t384: f64, t359: f64, t999: f64, t1032: f64, t1035: f64, t355: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1068, t1071) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk332(t1066, t906, t247, t1003, t1009, t1011, t1017, t1021, t1025, t1028, t1041, t1047, t1054, t1060, t1063, t348, t375);
        let (t1073, t1076) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk333(t1071, t225, t385, t342, t378);
        let (t1077, t1078, t1079) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk334(t384, t225);
        let t1082 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk335(t359, t378);
        let (t1083, t1086) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk336(t1082, t999, t1032, t1035);
        let t1087 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk337(t1086, t342);
        let t1089 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk338(t355, t357);
    (t1068, t1071, t1073, t1076, t1077, t1078, t1079, t1082, t1083, t1086, t1087, t1089)
}
