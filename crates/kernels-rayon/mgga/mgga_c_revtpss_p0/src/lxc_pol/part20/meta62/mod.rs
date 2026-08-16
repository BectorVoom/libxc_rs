//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta62 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk413;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk414;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk415;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk416;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk417;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk418;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta62(t1263: f64, t66: f64, t1122: f64, t247: f64, t1221: f64, t1222: f64, t1227: f64, t1231: f64, t1235: f64, t1238: f64, t1247: f64, t1252: f64, t1258: f64, t1261: f64, t484: f64, t225: f64, t494: f64, t460: f64, t487: f64, t493: f64, t473: f64, t1214: f64, t1032: f64, t1243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1264 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk413(t1263, t66);
        let (t1266, t1269) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk414(t1122, t1264, t247, t1221, t1222, t1227, t1231, t1235, t1238, t1247, t1252, t1258, t1261, t484);
        let (t1271, t1274) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk415(t1269, t225, t494, t460, t487);
        let (t1275, t1276, t1277) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk416(t493, t225);
        let t1280 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk417(t473, t487);
        let (t1281, t1284) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk418(t1214, t1280, t1032, t1243);
        let t1285 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk419(t1284, t460);
    (t1264, t1266, t1269, t1271, t1274, t1275, t1276, t1277, t1280, t1281, t1284, t1285)
}
