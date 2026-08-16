//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta54 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk350;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk351;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk352;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk353;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk354;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk355;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta54<F: Float>(t1044: F, t1045: F, t1042: F, t362: F, t39: F, t40: F, t361: F, t351: F, t127: F, t371: F, t373: F, t367: F, t365: F, t369: F, t270: F, t283: F, t66: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1046, t1047, t1052) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk350::<F>(t1044, t1045, t1042, t362, t39, t40);
        let t1053 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk351::<F>(t1052, t361);
        let (t1054, t1058) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk352::<F>(t1053, t351, t127, t371, t373);
        let (t1060, t1062) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk353::<F>(t1058, t367, t365, t369, t361);
        let t1063 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk354::<F>(t1062, t351);
        let t1065 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk355::<F>(t270, t283);
        let t1066 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk356::<F>(t1065, t66);
    (t1046, t1047, t1052, t1053, t1054, t1058, t1060, t1062, t1063, t1065, t1066)
}
