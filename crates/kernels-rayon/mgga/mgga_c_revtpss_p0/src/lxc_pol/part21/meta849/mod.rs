//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta849 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3189;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3190;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta849(t12571: f64, t5202: f64, t1196: f64, t16676: f64, t3516: f64, t12564: f64, t5192: f64, t17164: f64, t3531: f64, t1179: f64, t1188: f64, t58456: f64, t58598: f64, t58700: f64, t58703: f64, t58707: f64, t58711: f64, t58713: f64, t57817: f64, t57912: f64, t58472: f64, t58475: f64, t58477: f64, t58479: f64, t58481: f64, t58591: f64, t58686: f64, t58688: f64, t58690: f64, t58692: f64, t58695: f64, t1214: f64, t17784: f64, t1042: f64, t1122: f64, t1222: f64, t1247: f64, t1250: f64, t12621: f64, t12794: f64, t12809: f64, t12953: f64, t13102: f64, t16771: f64, t17505: f64, t17547: f64, t17736: f64, t247: f64, t3591: f64, t3626: f64, t3718: f64, t3719: f64, t3720: f64, t44675: f64, t44678: f64, t44681: f64, t471: f64, t482: f64, t5308: f64, t5312: f64, t5332: f64, t5351: f64, t5373: f64, t5384: f64, t5391: f64, t56157: f64, t56161: f64, t56165: f64, t56192: f64, t56196: f64, t56555: f64, t57780: f64, t57786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58715, t58718, t58720, t58722, t58726) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3189(t12571, t5202, t1196, t16676, t3516, t12564, t5192, t17164, t3531, t1179, t1188, t58456);
        let t58730 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3190(t58598, t58700, t58703, t58707, t58711, t58713, t58715, t58718, t58720, t58722, t58726, t57817, t57912, t58472, t58475, t58477, t58479, t58481, t58591, t58686, t58688, t58690, t58692, t58695);
        let (t58760, t58772) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3191(t1214, t17784, t1042, t1122, t1222, t1247, t1250, t12621, t12794, t12809, t12953, t13102, t16771, t17505, t17547, t17736, t247, t3591, t3626, t3718, t3719, t3720, t44675, t44678, t44681, t471, t482, t5308, t5312, t5332, t5351, t5373, t5384, t5391, t56157, t56161, t56165, t56192, t56196, t56555, t57780, t57786, t58730);
    (t58715, t58718, t58720, t58722, t58726, t58730, t58760, t58772)
}
