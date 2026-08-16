//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1424;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1425;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1426;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1427;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta407(t30: f64, t1450: f64, t6922: f64, t6785: f64, t9605: f64, t3874: f64, t5824: f64, t1344: f64, t18280: f64, t2255: f64, t5574: f64, t605: f64, t6792: f64, t9617: f64, zeta_threshold: f64, t33: f64, t3881: f64, t6416: f64, t1113: f64, t1348: f64, t20256: f64, t5582: f64, t1882: f64, t1892: f64, t4003: f64, t5658: f64, t10032: f64, t10035: f64, t10044: f64, t1399: f64, t14116: f64, t14120: f64, t14126: f64, t14131: f64, t14146: f64, t14149: f64, t14158: f64, t14161: f64, t14166: f64, t4118: f64, t5735: f64, t5745: f64, t5755: f64, t6844: f64, t820: f64, t555: f64, t6861: f64, t6843: f64, t1398: f64, t9994: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t9934: f64, t3989: f64, t6856: f64, t13762: f64, t13763: f64, t13765: f64, t13772: f64, t13778: f64, t9711: f64, t9712: f64, t9725: f64, t9729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21937, t21955, t21956) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1424(t30, t1450, t6922, t6785, t9605, t3874, t5824, t1344, t18280, t2255, t5574, t605, t6792, t9617, zeta_threshold);
        let (t21969, t21981, t21990) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1425(t33, t3881, t6416, t1113, t1348, t20256, t21956, t2255, t5582, t21955, t1882, t1892, t4003, t5658, zeta_threshold);
        let t21998 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1426(t10032, t10035, t10044, t1399, t14116, t14120, t14126, t14131, t14146, t14149, t14158, t14161, t14166, t21981, t21990, t4118, t5735, t5745, t5755, t6844, t820);
        let (t22005, t22009, t22016, t22023, t22025, t22028, t22030) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1427(t555, t6861, t6843, t1398, t9994, t550, t543, t3992, t2661, t4003, t9934, t3989, t6856);
        let t22035 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1428(t13762, t13763, t13765, t13772, t13778, t22023, t22028, t22030, t9711, t9712, t9725, t9729);
    (t21937, t21969, t21981, t21990, t21998, t22005, t22009, t22016, t22025, t22035)
}
