//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta58 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk371;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk372;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk373;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk374;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk375;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk376;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta58(t635: f64, t606: f64, t1120: f64, t128: f64, t1119: f64, t422: f64, t418: f64, t408: f64, t409: f64, t1118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1121 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk371(t635);
        let t1122 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk372(t1121, t606);
        let (t1123, t1124) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk373(t1120, t1122, t128);
        let (t1126, t1128, t1129, t1130) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk374(t1119, t1124, t422, t418);
        let t1131 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk375(t1130, t408);
        let t1132 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk376(t409);
        let t1134 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk377(t1118, t1124);
    (t1121, t1122, t1123, t1124, t1126, t1128, t1129, t1130, t1131, t1132, t1134)
}
