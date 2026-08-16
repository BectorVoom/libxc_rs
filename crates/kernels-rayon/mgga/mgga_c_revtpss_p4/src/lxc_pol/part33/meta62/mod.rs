//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta62 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk397;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk398;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk399;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk400;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk401;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk402;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk403;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk404;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta62(t225: f64, t494: f64, t1118: f64, t1124: f64, t139: f64, t221: f64, t462: f64, t461: f64, t1010: f64, t56: f64, t403: f64, t404: f64, t1121: f64, t606: f64, t1012: f64, t1204: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1211 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk397(t225, t494);
        let (t1212, t1214) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk398(t1118, t1124);
        let t1215 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk399(t1211, t1214);
        let t1219 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk400(t139, t221, t462);
        let (t1221, t1222) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk401(t1219, t461, t1010, t56);
        let t1224 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk402(t403, t404);
        let t1225 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk403(t1121, t1224);
        let (t1226, t1227) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk404(t1225, t606, t1012);
        let t1230 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk405(t1204, t225);
    (t1211, t1212, t1214, t1215, t1219, t1221, t1222, t1224, t1225, t1226, t1227, t1230)
}
