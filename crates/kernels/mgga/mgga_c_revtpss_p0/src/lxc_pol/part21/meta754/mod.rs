//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2638;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2639;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2640;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2641;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2642;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2643;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2644;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2645;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta754<F: Float>(t14055: F, t9775: F, t1885: F, t46722: F, t13867: F, t221: F, t3978: F, t9921: F, t14047: F, t14051: F, t1412: F, t5658: F, t2661: F, t3938: F, t3992: F, t14045: F, t9810: F, t13774: F, t1399: F, t13927: F, t48100: F, t9816: F, t1353: F, t13716: F, t13789: F, t1410: F, t3934: F, t4012: F, t46660: F, t48466: F, t48494: F, t48498: F, t48509: F, t48510: F, t48514: F, t5671: F, t5673: F, t5674: F, t828: F, t9912: F, t13910: F, t4057: F, t5651: F, t1389: F, t1882: F, t46856: F, t543: F, t685: F, t72: F, t13874: F, t3989: F, t13805: F, t46609: F, t5608: F, t4004: F, t9934: F, t13854: F, t9962: F, t13834: F, t13999: F, t125: F, t13920: F, t13955: F, t46946: F, t13775: F, t808: F, t9845: F, t13783: F, t13784: F, t13926: F, t13944: F, t36776: F, t3936: F, t46671: F, t46680: F, t48475: F, t46917: F, t5701: F, t14005: F, t46740: F, t5697: F, t46692: F, t46695: F, t46702: F, t46704: F, t46706: F, t46712: F, t46719: F, t46723: F, t46741: F, t46747: F, t46749: F, t5704: F, t9899: F, t1872: F, t9818: F, t13824: F, t46716: F, t13923: F, t3930: F, t14036: F, t9976: F, t1868: F, t1883: F, t46627: F, t46754: F, t46757: F, t46760: F, t46767: F, t46771: F, t46776: F, t46780: F, t46787: F, t46789: F, t46793: F, t46797: F, t9400: F, t9891: F, t9984: F, t46694: F, t5686: F, t14030: F, t9744: F, t13769: F, t9736: F, t13952: F, t2689: F, t46825: F, t9793: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48516, t48518, t48527, t48529, t48532, t48533) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2638::<F>(t14055, t9775, t1885, t46722, t13867, t221, t3978, t9921, t14047, t14051, t1412, t5658);
        let t48550 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2639::<F>(t2661, t3938, t3992, t48533, t14045, t9810, t13774, t1399, t13927, t48100, t9816, t1353, t13716, t13789, t1410, t3934, t4012, t46660, t48466, t48494, t48498, t48509, t48510, t48514, t48516, t48518, t48527, t48529, t48532, t5671, t5673, t5674, t828, t9912);
        let (t48553, t48557, t48563) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2640::<F>(t13910, t1399, t2661, t3992, t4057, t5651, t1389, t1882, t46856, t543, t685, t72);
        let (t48565, t48573, t48577, t48591, t48593) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2641::<F>(t13874, t3989, t13805, t2661, t46609, t5608, t4004, t9934, t13854, t9962, t13834, t13999);
        let (t48595, t48611) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2642::<F>(t125, t13920, t13955, t46946, t13775, t808, t9845, t13783, t13784, t13789, t13926, t13944, t1399, t36776, t3934, t3936, t3938, t46671, t46680, t48475, t48553, t48557, t48563, t48565, t48573, t48577, t48591, t48593, t9810);
        let t48647 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2643::<F>(t46917, t5701, t14005, t46740, t5697, t13944, t1399, t3934, t3936, t4004, t4057, t46692, t46695, t46702, t46704, t46706, t46712, t46719, t46723, t46741, t46747, t46749, t48595, t5671, t5673, t5704, t9899);
        let (t48655, t48664, t48666, t48668) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2644::<F>(t1872, t4057, t9816, t9818, t13824, t221, t3978, t46716, t13923, t3930, t14036, t9976);
        let t48683 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2645::<F>(t48668, t13783, t1410, t1868, t1883, t3934, t3936, t46627, t46754, t46757, t46760, t46767, t46771, t46776, t46780, t46787, t46789, t46793, t46797, t48655, t48664, t48666, t5704, t828, t9400, t9891, t9984);
        let (t48686, t48687, t48691, t48692, t48694, t48696) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2646::<F>(t46694, t5686, t14030, t9744, t13769, t808, t9736, t13952, t2689, t13784, t543, t46825, t9793);
    (t48550, t48595, t48611, t48647, t48683, t48686, t48687, t48691, t48692, t48694, t48696)
}
