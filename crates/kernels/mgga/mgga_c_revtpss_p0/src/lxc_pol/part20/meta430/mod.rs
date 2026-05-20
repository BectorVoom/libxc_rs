//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1618;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1619;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1620;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1621;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1622;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta430<F: Float>(t1214: F, t13045: F, t12804: F, t12916: F, t3718: F, t12854: F, t17350: F, t12808: F, t12865: F, t12909: F, t13051: F, t44173: F, t1261: F, t1264: F, t12780: F, t12800: F, t12805: F, t12822: F, t12828: F, t12832: F, t12841: F, t12846: F, t12858: F, t12866: F, t12867: F, t13055: F, t13079: F, t247: F, t3630: F, t3640: F, t3644: F, t3647: F, t372: F, t3720: F, t43797: F, t44484: F, t44500: F, t44501: F, t13037: F, t472: F, t44372: F, t44373: F, t474: F, t3603: F, t42871: F, t482: F, t675: F, t828: F, t3722: F, t3566: F, t3766: F, t5330: F, t3568: F, t3601: F, t12646: F, t12915: F, t5384: F, t12831: F, t1260: F, t12889: F, t12886: F, t1209: F, t13141: F, t17708: F, t12917: F, t11249: F, t3588: F, t1042: F, t1122: F, t12286: F, t1266: F, t12856: F, t12868: F, t12931: F, t12951: F, t17709: F, t17729: F, t17736: F, t3604: F, t3618: F, t3626: F, t43789: F, t44377: F, t1248: F, t12621: F, t371: F, t481: F, t9291: F, t12627: F, t1284: F, t3624: F, t12629: F, t12910: F, t12911: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44502, t44508, t44510, t44517, t44521, t44526) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1618::<F>(t1214, t13045, t12804, t12916, t3718, t12854, t17350, t12808, t12865, t12909, t13051, t44173);
        let t44529 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1619::<F>(t1261, t1264, t12780, t12800, t12805, t12822, t12828, t12832, t12841, t12846, t12858, t12866, t12867, t13055, t13079, t247, t3630, t3640, t3644, t3647, t372, t3720, t43797, t44484, t44500, t44501, t44502, t44508, t44510, t44517, t44521, t44526);
        let (t44531, t44534, t44535, t44536, t44548) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1620::<F>(t13037, t472, t44372, t44373, t474, t3603, t42871, t482, t675, t828, t3718, t3722);
        let (t44551, t44552, t44559, t44561, t44568) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1621::<F>(t3566, t3766, t5330, t3568, t3601, t12646, t12915, t247, t5384, t12831, t12865, t1260, t12889);
        let (t44585, t44595) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1622::<F>(t12886, t3647, t1209, t13141, t17708, t12832, t12917, t11249, t3601, t13045, t3588, t1042, t1122, t12286, t1261, t12646, t1266, t12856, t12866, t12868, t12931, t12951, t17709, t17729, t17736, t247, t3604, t3618, t3626, t3630, t372, t3720, t43789, t44377, t44501, t44534, t44536, t44548, t44551, t44552, t44559, t44561, t44568);
        let (t44599, t44607, t44609, t44610, t44616) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1623::<F>(t1248, t12621, t371, t481, t482, t9291, t12627, t1284, t3624, t12629, t12910, t12911, t12916);
    (t44529, t44531, t44535, t44552, t44585, t44595, t44599, t44607, t44609, t44610, t44616)
}
