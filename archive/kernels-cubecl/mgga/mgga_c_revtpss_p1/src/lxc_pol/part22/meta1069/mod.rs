//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1069 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3822;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3823;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3824;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3825;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1069<F: Float>(t162: F, t73444: F, t73470: F, t189: F, t512: F, t21931: F, t749: F, t22212: F, t2516: F, t1868: F, t4144: F, t72: F, t757: F, t13625: F, t13674: F, t1907: F, t198: F, t33596: F, t39799: F, t39807: F, t39813: F, t4139: F, t47059: F, t49647: F, t530: F, t73418: F, t6922: F, t9593: F, t22185: F, t2619: F, t48277: F, t47672: F, t6781: F, t13600: F, t13716: F, t13867: F, t13872: F, t22475: F, t47067: F, t5532: F, t5536: F, t5537: F, t5541: F, t5627: F, t6836: F, t9547: F, t30: F, t13687: F, t14: F, t18280: F, t21944: F, t21949: F, t2257: F, t27: F, t3834: F, t3874: F, t46310: F, t48394: F, t5574: F, t580: F, t5824: F, t605: F, t6785: F, t73423: F, t9342: F, t9605: F, zeta_threshold: F, t33: F, t1113: F, t13701: F, t20256: F, t21956: F, t21961: F, t3351: F, t3842: F, t3881: F, t46328: F, t48417: F, t5582: F, t6416: F, t6792: F, t73449: F, t9617: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73472, t73474, t73477, t73482, t73488, t73493) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3822::<F>(t162, t73444, t73470, t189, t512, t21931, t749, t22212, t2516, t1868, t4144, t72, t757);
        let (t73494, t73495) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3823::<F>(t73493, t13625, t13674, t1868, t1907, t198, t33596, t39799, t39807, t39813, t4139, t47059, t49647, t530, t73418, t73474, t73477, t73482, t73488);
        let (t73516, t73517, t73528) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3824::<F>(t6922, t9593, t22185, t2619, t48277, t47672, t6781, t13600, t13625, t13716, t13867, t13872, t22475, t4139, t4144, t47067, t5532, t5536, t5537, t5541, t5627, t6836, t9547);
        let t73552 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3825::<F>(t30, t13687, t14, t18280, t21944, t21949, t2257, t27, t3834, t3874, t46310, t48394, t5574, t580, t5824, t605, t6785, t73423, t9342, t9605, zeta_threshold);
        let t73576 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3826::<F>(t33, t1113, t13701, t14, t20256, t21956, t21961, t27, t3351, t3842, t3881, t46328, t48417, t5582, t580, t6416, t6792, t73449, t9342, t9617, zeta_threshold);
    (t73472, t73474, t73477, t73482, t73494, t73495, t73516, t73517, t73528, t73552, t73576)
}
