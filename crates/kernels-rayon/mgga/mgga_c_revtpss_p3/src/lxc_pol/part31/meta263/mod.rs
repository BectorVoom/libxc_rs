//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta263 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1173;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1174;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1175;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1176;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1177;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta263(t7143: f64, t7150: f64, t1976: f64, t999: f64, t7145: f64, t1071: f64, t1982: f64, t3268: f64, t359: f64, t1096: f64, t3140: f64, t378: f64, t1078: f64, t1035: f64, t1043: f64, t1089: f64, t1984: f64, t7135: f64, t1000: f64, t1097: f64, t1978: f64, t1983: f64, t1986: f64, t342: f64, t7102: f64, t7137: f64, t7140: f64, t7144: f64, t7147: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7151 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1173(t7143, t7150);
        let (t7153, t7156, t7159) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1174(t1976, t999, t7145, t1071, t1982, t7143);
        let t7160 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1175(t3268, t359);
        let (t7162, t7166, t7167) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1176(t1096, t1976, t7160, t3140, t378, t1078, t1982);
        let t7168 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1177(t1035, t1976);
        let (t7170, t7174, t7177) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1178(t1043, t1089, t7168, t1984, t359, t7135, t1000, t1097, t1978, t1983, t1986, t342, t7102, t7137, t7140, t7144, t7147, t7151, t7153, t7156, t7159, t7162, t7167, t989);
    (t7151, t7153, t7156, t7159, t7160, t7162, t7166, t7167, t7168, t7170, t7174, t7177)
}
