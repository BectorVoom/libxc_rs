//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta688 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2679;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2680;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2681;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2682;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2683;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2684;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta688(t30: f64, t187: f64, t21931: f64, t1450: f64, t6922: f64, t6785: f64, t9605: f64, t3874: f64, t5824: f64, t1344: f64, t18280: f64, t2255: f64, t5574: f64, t605: f64, zeta_threshold: f64, t33: f64, t6792: f64, t9617: f64, t3881: f64, t6416: f64, t1113: f64, t1348: f64, t20256: f64, t5582: f64, t1882: f64, t1892: f64, t4003: f64, t5658: f64, t10032: f64, t10035: f64, t10044: f64, t1399: f64, t14116: f64, t14120: f64, t14126: f64, t14131: f64, t14146: f64, t14149: f64, t14158: f64, t14161: f64, t14166: f64, t4118: f64, t5735: f64, t5745: f64, t5755: f64, t6844: f64, t820: f64, t555: f64, t6861: f64, t6843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21933, t21937, t21944, t21949, t21955) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2679(t30, t187, t21931, t1450, t6922, t6785, t9605, t3874, t5824, t1344, t18280, t2255, t5574, t605, zeta_threshold);
        let (t21956, t21961, t21969) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2680(t33, t6792, t9617, t3881, t6416, t1113, t1348, t20256, t2255, t5582, t21955, zeta_threshold);
        let t21981 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2681(t1882, t1892);
        let t21990 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2682(t4003, t5658);
        let t21998 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2683(t10032, t10035, t10044, t1399, t14116, t14120, t14126, t14131, t14146, t14149, t14158, t14161, t14166, t21981, t21990, t4118, t5735, t5745, t5755, t6844, t820);
        let t22005 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2684(t555, t6861);
        let t22009 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2685(t555, t6843);
    (t21933, t21937, t21944, t21949, t21956, t21961, t21969, t21981, t21990, t21998, t22005, t22009)
}
