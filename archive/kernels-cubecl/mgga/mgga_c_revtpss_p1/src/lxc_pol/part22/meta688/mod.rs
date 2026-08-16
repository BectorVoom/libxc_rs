//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta688 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2679;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2680;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2681;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2682;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2683;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2684;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta688<F: Float>(t30: F, t187: F, t21931: F, t1450: F, t6922: F, t6785: F, t9605: F, t3874: F, t5824: F, t1344: F, t18280: F, t2255: F, t5574: F, t605: F, zeta_threshold: F, t33: F, t6792: F, t9617: F, t3881: F, t6416: F, t1113: F, t1348: F, t20256: F, t5582: F, t1882: F, t1892: F, t4003: F, t5658: F, t10032: F, t10035: F, t10044: F, t1399: F, t14116: F, t14120: F, t14126: F, t14131: F, t14146: F, t14149: F, t14158: F, t14161: F, t14166: F, t4118: F, t5735: F, t5745: F, t5755: F, t6844: F, t820: F, t555: F, t6861: F, t6843: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21933, t21937, t21944, t21949, t21955) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2679::<F>(t30, t187, t21931, t1450, t6922, t6785, t9605, t3874, t5824, t1344, t18280, t2255, t5574, t605, zeta_threshold);
        let (t21956, t21961, t21969) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2680::<F>(t33, t6792, t9617, t3881, t6416, t1113, t1348, t20256, t2255, t5582, t21955, zeta_threshold);
        let t21981 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2681::<F>(t1882, t1892);
        let t21990 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2682::<F>(t4003, t5658);
        let t21998 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2683::<F>(t10032, t10035, t10044, t1399, t14116, t14120, t14126, t14131, t14146, t14149, t14158, t14161, t14166, t21981, t21990, t4118, t5735, t5745, t5755, t6844, t820);
        let t22005 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2684::<F>(t555, t6861);
        let t22009 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2685::<F>(t555, t6843);
    (t21933, t21937, t21944, t21949, t21956, t21961, t21969, t21981, t21990, t21998, t22005, t22009)
}
