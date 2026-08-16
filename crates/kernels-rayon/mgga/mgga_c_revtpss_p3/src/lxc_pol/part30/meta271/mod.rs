//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta271 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1192;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1193;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1194;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1195;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1196;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1197;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1198;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1199;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta271(t1243: f64, t1245: f64, sigma2: f64, t1241: f64, t1256: f64, t2139: f64, t1259: f64, t2137: f64, t467: f64, t1227: f64, t1238: f64, t1252: f64, t1266: f64, t484: f64, t7606: f64, t7607: f64, t7610: f64, t7613: f64, t225: f64, t494: f64, t2142: f64, t460: f64, t1032: f64, t487: f64, t1209: f64, t1276: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7616, t7617) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1192(t1243, t1245, sigma2);
        let t7618 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1193(t1241, t7617);
        let (t7622, t7623) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1194(t1256, t2139, t1259, t2137);
        let t7624 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1195(t467, t7623);
        let t7627 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1196(t1227, t1238, t1252, t1266, t484, t7606, t7607, t7610, t7613, t7618, t7622, t7624);
        let (t7629, t7632) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1197(t225, t494, t7627, t2142, t460);
        let t7635 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1198(t1032, t487);
        let t7636 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1199(t1209, t7635);
        let t7637 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1200(t1276, t473);
    (t7616, t7617, t7618, t7622, t7623, t7624, t7627, t7629, t7632, t7635, t7636, t7637)
}
