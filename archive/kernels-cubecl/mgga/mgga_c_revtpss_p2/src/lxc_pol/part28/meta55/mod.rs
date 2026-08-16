//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta55 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk361;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk362;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk363;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk364;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk365;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk366;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk367;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk368;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta55<F: Float>(t362: F, t39: F, t40: F, t361: F, t351: F, t127: F, t371: F, t373: F, t367: F, t365: F, t369: F, t270: F, t283: F, t66: F, t906: F, t247: F, t1003: F, t1009: F, t1011: F, t1017: F, t1021: F, t1025: F, t1028: F, t1041: F, t1047: F, t348: F, t375: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1052 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk361::<F>(t362, t39, t40);
        let t1053 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk362::<F>(t1052, t361);
        let (t1054, t1058) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk363::<F>(t1053, t351, t127, t371, t373);
        let (t1060, t1061, t1062) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk364::<F>(t1058, t367, t365, t369, t361);
        let t1063 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk365::<F>(t1062, t351);
        let t1065 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk366::<F>(t270, t283);
        let t1066 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk367::<F>(t1065, t66);
        let t1068 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk368::<F>(t1066, t906, t247);
        let t1071 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk369::<F>(t1003, t1009, t1011, t1017, t1021, t1025, t1028, t1041, t1047, t1054, t1060, t1063, t1068, t348, t375);
    (t1052, t1053, t1054, t1058, t1060, t1061, t1062, t1063, t1065, t1066, t1068, t1071)
}
