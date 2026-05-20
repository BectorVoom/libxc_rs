//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta849 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3189;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3190;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta849<F: Float>(t12571: F, t5202: F, t1196: F, t16676: F, t3516: F, t12564: F, t5192: F, t17164: F, t3531: F, t1179: F, t1188: F, t58456: F, t58598: F, t58700: F, t58703: F, t58707: F, t58711: F, t58713: F, t57817: F, t57912: F, t58472: F, t58475: F, t58477: F, t58479: F, t58481: F, t58591: F, t58686: F, t58688: F, t58690: F, t58692: F, t58695: F, t1214: F, t17784: F, t1042: F, t1122: F, t1222: F, t1247: F, t1250: F, t12621: F, t12794: F, t12809: F, t12953: F, t13102: F, t16771: F, t17505: F, t17547: F, t17736: F, t247: F, t3591: F, t3626: F, t3718: F, t3719: F, t3720: F, t44675: F, t44678: F, t44681: F, t471: F, t482: F, t5308: F, t5312: F, t5332: F, t5351: F, t5373: F, t5384: F, t5391: F, t56157: F, t56161: F, t56165: F, t56192: F, t56196: F, t56555: F, t57780: F, t57786: F) -> (F, F, F, F, F, F, F, F) {
        let (t58715, t58718, t58720, t58722, t58726) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3189::<F>(t12571, t5202, t1196, t16676, t3516, t12564, t5192, t17164, t3531, t1179, t1188, t58456);
        let t58730 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3190::<F>(t58598, t58700, t58703, t58707, t58711, t58713, t58715, t58718, t58720, t58722, t58726, t57817, t57912, t58472, t58475, t58477, t58479, t58481, t58591, t58686, t58688, t58690, t58692, t58695);
        let (t58760, t58772) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3191::<F>(t1214, t17784, t1042, t1122, t1222, t1247, t1250, t12621, t12794, t12809, t12953, t13102, t16771, t17505, t17547, t17736, t247, t3591, t3626, t3718, t3719, t3720, t44675, t44678, t44681, t471, t482, t5308, t5312, t5332, t5351, t5373, t5384, t5391, t56157, t56161, t56165, t56192, t56196, t56555, t57780, t57786, t58730);
    (t58715, t58718, t58720, t58722, t58726, t58730, t58760, t58772)
}
