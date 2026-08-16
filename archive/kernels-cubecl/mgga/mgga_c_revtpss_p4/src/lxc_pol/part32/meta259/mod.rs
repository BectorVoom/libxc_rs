//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta259 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1087;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1088;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1089;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1090;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1091;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1092;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1093;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1094;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta259<F: Float>(t1416: F, t7271: F, t1032: F, t555: F, t1426: F, t786: F, t7063: F, t1419: F, t1955: F, t4075: F, t545: F, t1385: F, t1448: F, t4147: F, t38: F, t68: F, t2247: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7272, t7282) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1087::<F>(t1416, t7271, t1032, t555);
        let t7283 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1088::<F>(t1426, t7282);
        let t7284 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1089::<F>(t7283, t786);
        let t7289 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1090::<F>(t7063, t7283);
        let (t7292, t7295) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1091::<F>(t1419, t1955, t7282);
        let t7296 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1092::<F>(t4075, t545);
        let t7301 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1093::<F>(t1385, t1426);
        let (t7315, t7342) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1094::<F>(t1448, t4147, t38, t68);
        let t7343 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1095::<F>(t2247, t7342);
    (t7272, t7282, t7283, t7284, t7289, t7292, t7295, t7296, t7301, t7315, t7342, t7343)
}
