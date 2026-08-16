//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta56 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk341;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk342;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk343;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk344;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk345;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta56(t1071: f64, t225: f64, t385: f64, t342: f64, t378: f64, t384: f64, t359: f64, t999: f64, t1032: f64, t1035: f64, t355: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1073, t1076) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk341(t1071, t225, t385, t342, t378);
        let (t1077, t1078, t1079) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk342(t384, t225);
        let t1082 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk343(t359, t378);
        let (t1083, t1086) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk344(t1082, t999, t1032, t1035);
        let t1087 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk345(t1086, t342);
        let t1089 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk346(t355, t357);
    (t1073, t1076, t1077, t1078, t1079, t1082, t1083, t1086, t1087, t1089)
}
