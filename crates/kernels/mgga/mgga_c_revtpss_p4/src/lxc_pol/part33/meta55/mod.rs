//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta55 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk357;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk358;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk359;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk360;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk361;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk362;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta55<F: Float>(t1066: F, t906: F, t247: F, t1003: F, t1009: F, t1011: F, t1017: F, t1021: F, t1025: F, t1028: F, t1041: F, t1047: F, t1054: F, t1060: F, t1063: F, t348: F, t375: F, t225: F, t385: F, t342: F, t378: F, t384: F, t359: F, t999: F, t1032: F, t1035: F, t355: F, t357: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1068, t1071) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk357::<F>(t1066, t906, t247, t1003, t1009, t1011, t1017, t1021, t1025, t1028, t1041, t1047, t1054, t1060, t1063, t348, t375);
        let (t1073, t1076) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk358::<F>(t1071, t225, t385, t342, t378);
        let (t1077, t1078, t1079) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk359::<F>(t384, t225);
        let t1082 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk360::<F>(t359, t378);
        let (t1083, t1086) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk361::<F>(t1082, t999, t1032, t1035);
        let t1087 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk362::<F>(t1086, t342);
        let t1089 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk363::<F>(t355, t357);
    (t1068, t1071, t1073, t1076, t1077, t1078, t1079, t1082, t1083, t1086, t1087, t1089)
}
