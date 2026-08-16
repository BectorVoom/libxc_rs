//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1781;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1782;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1783;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta578(t1196: f64, t45187: f64, t45190: f64, t90357: f64, t90602: f64, t90629: f64, t90631: f64, t90634: f64, t90636: f64, t90640: f64, t90644: f64, t90855: f64, t90857: f64, t90860: f64, t90863: f64, t90347: f64, t90506: f64, t90600: f64, t23842: f64, t24792: f64, t24610: f64, t1715: f64, t1774: f64, t6622: f64, t1042: f64, t1247: f64, t1250: f64, t12866: f64, t17235: f64, t17351: f64, t17353: f64, t17693: f64, t17799: f64, t20795: f64, t24773: f64, t3604: f64, t3611: f64, t3626: f64, t3711: f64, t44458: f64, t44510: f64, t482: f64, t5274: f64, t5340: f64, t5819: f64, t69839: f64, t69910: f64, t69964: f64, t82932: f64, t90001: f64, t24633: f64, t17401: f64, t247: f64, t24744: f64, t24753: f64, t24846: f64, t3719: f64, t3720: f64, t44551: f64, t5384: f64, t5391: f64, t57660: f64, t6640: f64, t6690: f64, t70032: f64, t70995: f64, t71081: f64, t83018: f64, t83047: f64, t83067: f64, t89978: f64, t17396: f64, t1791: f64, t21014: f64, t24729: f64, t24731: f64, t24734: f64, t24741: f64, t24840: f64, t3671: f64, t371: f64, t372: f64, t5331: f64, t57710: f64, t59411: f64, t70112: f64, t70133: f64, t82859: f64, t83114: f64, t83158: f64, t89808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90867, t90868) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1781(t1196, t45187, t45190, t90357, t90602, t90629, t90631, t90634, t90636, t90640, t90644, t90855, t90857, t90860, t90863);
        let (t90870, t90881, t90885, t90889, t90894, t90900) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1782(t90347, t90506, t90600, t90868, t23842, t24792, t24610, t1715, t1774, t6622, t1042, t1247, t1250, t12866, t17235, t17351, t17353, t17693, t17799, t20795, t24773, t3604, t3611, t3626, t3711, t44458, t44510, t482, t5274, t5340, t5819, t69839, t69910, t69964, t82932, t90001);
        let (t90926, t90946) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1783(t1774, t24633, t17401, t247, t24744, t24753, t24846, t3604, t3719, t3720, t44551, t5384, t5391, t57660, t6640, t6690, t70032, t70995, t71081, t83018, t83047, t83067, t89978);
        let t90998 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1784(t17396, t1791, t21014, t24729, t24731, t24734, t24741, t24753, t24840, t3671, t371, t372, t3720, t482, t5331, t5340, t57710, t59411, t70112, t70133, t82859, t83114, t83158, t89808);
    (t90867, t90870, t90881, t90885, t90889, t90894, t90900, t90926, t90946, t90998)
}
