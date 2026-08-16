//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta56 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk397;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk398;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk399;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk400;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk401;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk402;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk403;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta56(t635: f64, t606: f64, t1120: f64, t128: f64, t1119: f64, t422: f64, t418: f64, t408: f64, t409: f64, t1118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1121 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk397(t635);
        let t1122 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk398(t1121, t606);
        let (t1123, t1124) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk399(t1120, t1122, t128);
        let t1126 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk400(t1119, t1124);
        let (t1128, t1129, t1130) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk401(t1126, t422, t418);
        let t1131 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk402(t1130, t408);
        let t1132 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk403(t409);
        let t1134 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk404(t1118, t1124);
    (t1121, t1122, t1123, t1124, t1126, t1128, t1129, t1130, t1131, t1132, t1134)
}
