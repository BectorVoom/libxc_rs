//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1781;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1782;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1783;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta578<F: Float>(t1196: F, t45187: F, t45190: F, t90357: F, t90602: F, t90629: F, t90631: F, t90634: F, t90636: F, t90640: F, t90644: F, t90855: F, t90857: F, t90860: F, t90863: F, t90347: F, t90506: F, t90600: F, t23842: F, t24792: F, t24610: F, t1715: F, t1774: F, t6622: F, t1042: F, t1247: F, t1250: F, t12866: F, t17235: F, t17351: F, t17353: F, t17693: F, t17799: F, t20795: F, t24773: F, t3604: F, t3611: F, t3626: F, t3711: F, t44458: F, t44510: F, t482: F, t5274: F, t5340: F, t5819: F, t69839: F, t69910: F, t69964: F, t82932: F, t90001: F, t24633: F, t17401: F, t247: F, t24744: F, t24753: F, t24846: F, t3719: F, t3720: F, t44551: F, t5384: F, t5391: F, t57660: F, t6640: F, t6690: F, t70032: F, t70995: F, t71081: F, t83018: F, t83047: F, t83067: F, t89978: F, t17396: F, t1791: F, t21014: F, t24729: F, t24731: F, t24734: F, t24741: F, t24840: F, t3671: F, t371: F, t372: F, t5331: F, t57710: F, t59411: F, t70112: F, t70133: F, t82859: F, t83114: F, t83158: F, t89808: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90867, t90868) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1781::<F>(t1196, t45187, t45190, t90357, t90602, t90629, t90631, t90634, t90636, t90640, t90644, t90855, t90857, t90860, t90863);
        let (t90870, t90881, t90885, t90889, t90894, t90900) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1782::<F>(t90347, t90506, t90600, t90868, t23842, t24792, t24610, t1715, t1774, t6622, t1042, t1247, t1250, t12866, t17235, t17351, t17353, t17693, t17799, t20795, t24773, t3604, t3611, t3626, t3711, t44458, t44510, t482, t5274, t5340, t5819, t69839, t69910, t69964, t82932, t90001);
        let (t90926, t90946) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1783::<F>(t1774, t24633, t17401, t247, t24744, t24753, t24846, t3604, t3719, t3720, t44551, t5384, t5391, t57660, t6640, t6690, t70032, t70995, t71081, t83018, t83047, t83067, t89978);
        let t90998 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1784::<F>(t17396, t1791, t21014, t24729, t24731, t24734, t24741, t24753, t24840, t3671, t371, t372, t3720, t482, t5331, t5340, t57710, t59411, t70112, t70133, t82859, t83114, t83158, t89808);
    (t90867, t90870, t90881, t90885, t90889, t90894, t90900, t90926, t90946, t90998)
}
