//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1173;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1174;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1175;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1176;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1177;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta263<F: Float>(t7143: F, t7150: F, t1976: F, t999: F, t7145: F, t1071: F, t1982: F, t3268: F, t359: F, t1096: F, t3140: F, t378: F, t1078: F, t1035: F, t1043: F, t1089: F, t1984: F, t7135: F, t1000: F, t1097: F, t1978: F, t1983: F, t1986: F, t342: F, t7102: F, t7137: F, t7140: F, t7144: F, t7147: F, t989: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t7151 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1173::<F>(t7143, t7150);
        let (t7153, t7156, t7159) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1174::<F>(t1976, t999, t7145, t1071, t1982, t7143);
        let t7160 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1175::<F>(t3268, t359);
        let (t7162, t7166, t7167) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1176::<F>(t1096, t1976, t7160, t3140, t378, t1078, t1982);
        let t7168 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1177::<F>(t1035, t1976);
        let (t7170, t7174, t7177) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1178::<F>(t1043, t1089, t7168, t1984, t359, t7135, t1000, t1097, t1978, t1983, t1986, t342, t7102, t7137, t7140, t7144, t7147, t7151, t7153, t7156, t7159, t7162, t7167, t989);
    (t7151, t7153, t7156, t7159, t7160, t7162, t7166, t7167, t7168, t7170, t7174, t7177)
}
