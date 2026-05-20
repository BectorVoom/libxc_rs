//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk422;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk423;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk424;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk425;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk426;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk427;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk428;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta65<F: Float>(t1221: F, t1222: F, t1227: F, t1231: F, t1235: F, t1238: F, t1247: F, t1252: F, t1258: F, t1261: F, t1266: F, t484: F, t225: F, t494: F, t460: F, t487: F, t493: F, t473: F, t1214: F, t1032: F, t1243: F, t355: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1269 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk422::<F>(t1221, t1222, t1227, t1231, t1235, t1238, t1247, t1252, t1258, t1261, t1266, t484);
        let (t1271, t1274) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk423::<F>(t1269, t225, t494, t460, t487);
        let (t1275, t1276) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk424::<F>(t493);
        let t1277 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk425::<F>(t1276, t225);
        let t1280 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk426::<F>(t473, t487);
        let (t1281, t1284) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk427::<F>(t1214, t1280, t1032, t1243);
        let t1285 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk428::<F>(t1284, t460);
        let t1287 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk429::<F>(t355, t471);
    (t1269, t1271, t1274, t1275, t1276, t1277, t1280, t1281, t1284, t1285, t1287)
}
