//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1069 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3822;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3823;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3824;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3825;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1069(t162: f64, t73444: f64, t73470: f64, t189: f64, t512: f64, t21931: f64, t749: f64, t22212: f64, t2516: f64, t1868: f64, t4144: f64, t72: f64, t757: f64, t13625: f64, t13674: f64, t1907: f64, t198: f64, t33596: f64, t39799: f64, t39807: f64, t39813: f64, t4139: f64, t47059: f64, t49647: f64, t530: f64, t73418: f64, t6922: f64, t9593: f64, t22185: f64, t2619: f64, t48277: f64, t47672: f64, t6781: f64, t13600: f64, t13716: f64, t13867: f64, t13872: f64, t22475: f64, t47067: f64, t5532: f64, t5536: f64, t5537: f64, t5541: f64, t5627: f64, t6836: f64, t9547: f64, t30: f64, t13687: f64, t14: f64, t18280: f64, t21944: f64, t21949: f64, t2257: f64, t27: f64, t3834: f64, t3874: f64, t46310: f64, t48394: f64, t5574: f64, t580: f64, t5824: f64, t605: f64, t6785: f64, t73423: f64, t9342: f64, t9605: f64, zeta_threshold: f64, t33: f64, t1113: f64, t13701: f64, t20256: f64, t21956: f64, t21961: f64, t3351: f64, t3842: f64, t3881: f64, t46328: f64, t48417: f64, t5582: f64, t6416: f64, t6792: f64, t73449: f64, t9617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73472, t73474, t73477, t73482, t73488, t73493) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3822(t162, t73444, t73470, t189, t512, t21931, t749, t22212, t2516, t1868, t4144, t72, t757);
        let (t73494, t73495) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3823(t73493, t13625, t13674, t1868, t1907, t198, t33596, t39799, t39807, t39813, t4139, t47059, t49647, t530, t73418, t73474, t73477, t73482, t73488);
        let (t73516, t73517, t73528) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3824(t6922, t9593, t22185, t2619, t48277, t47672, t6781, t13600, t13625, t13716, t13867, t13872, t22475, t4139, t4144, t47067, t5532, t5536, t5537, t5541, t5627, t6836, t9547);
        let t73552 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3825(t30, t13687, t14, t18280, t21944, t21949, t2257, t27, t3834, t3874, t46310, t48394, t5574, t580, t5824, t605, t6785, t73423, t9342, t9605, zeta_threshold);
        let t73576 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3826(t33, t1113, t13701, t14, t20256, t21956, t21961, t27, t3351, t3842, t3881, t46328, t48417, t5582, t580, t6416, t6792, t73449, t9342, t9617, zeta_threshold);
    (t73472, t73474, t73477, t73482, t73494, t73495, t73516, t73517, t73528, t73552, t73576)
}
