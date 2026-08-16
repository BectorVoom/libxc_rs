//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1618;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1619;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1620;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1621;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1622;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta430(t1214: f64, t13045: f64, t12804: f64, t12916: f64, t3718: f64, t12854: f64, t17350: f64, t12808: f64, t12865: f64, t12909: f64, t13051: f64, t44173: f64, t1261: f64, t1264: f64, t12780: f64, t12800: f64, t12805: f64, t12822: f64, t12828: f64, t12832: f64, t12841: f64, t12846: f64, t12858: f64, t12866: f64, t12867: f64, t13055: f64, t13079: f64, t247: f64, t3630: f64, t3640: f64, t3644: f64, t3647: f64, t372: f64, t3720: f64, t43797: f64, t44484: f64, t44500: f64, t44501: f64, t13037: f64, t472: f64, t44372: f64, t44373: f64, t474: f64, t3603: f64, t42871: f64, t482: f64, t675: f64, t828: f64, t3722: f64, t3566: f64, t3766: f64, t5330: f64, t3568: f64, t3601: f64, t12646: f64, t12915: f64, t5384: f64, t12831: f64, t1260: f64, t12889: f64, t12886: f64, t1209: f64, t13141: f64, t17708: f64, t12917: f64, t11249: f64, t3588: f64, t1042: f64, t1122: f64, t12286: f64, t1266: f64, t12856: f64, t12868: f64, t12931: f64, t12951: f64, t17709: f64, t17729: f64, t17736: f64, t3604: f64, t3618: f64, t3626: f64, t43789: f64, t44377: f64, t1248: f64, t12621: f64, t371: f64, t481: f64, t9291: f64, t12627: f64, t1284: f64, t3624: f64, t12629: f64, t12910: f64, t12911: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44502, t44508, t44510, t44517, t44521, t44526) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1618(t1214, t13045, t12804, t12916, t3718, t12854, t17350, t12808, t12865, t12909, t13051, t44173);
        let t44529 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1619(t1261, t1264, t12780, t12800, t12805, t12822, t12828, t12832, t12841, t12846, t12858, t12866, t12867, t13055, t13079, t247, t3630, t3640, t3644, t3647, t372, t3720, t43797, t44484, t44500, t44501, t44502, t44508, t44510, t44517, t44521, t44526);
        let (t44531, t44534, t44535, t44536, t44548) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1620(t13037, t472, t44372, t44373, t474, t3603, t42871, t482, t675, t828, t3718, t3722);
        let (t44551, t44552, t44559, t44561, t44568) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1621(t3566, t3766, t5330, t3568, t3601, t12646, t12915, t247, t5384, t12831, t12865, t1260, t12889);
        let (t44585, t44595) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1622(t12886, t3647, t1209, t13141, t17708, t12832, t12917, t11249, t3601, t13045, t3588, t1042, t1122, t12286, t1261, t12646, t1266, t12856, t12866, t12868, t12931, t12951, t17709, t17729, t17736, t247, t3604, t3618, t3626, t3630, t372, t3720, t43789, t44377, t44501, t44534, t44536, t44548, t44551, t44552, t44559, t44561, t44568);
        let (t44599, t44607, t44609, t44610, t44616) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1623(t1248, t12621, t371, t481, t482, t9291, t12627, t1284, t3624, t12629, t12910, t12911, t12916);
    (t44529, t44531, t44535, t44552, t44585, t44595, t44599, t44607, t44609, t44610, t44616)
}
