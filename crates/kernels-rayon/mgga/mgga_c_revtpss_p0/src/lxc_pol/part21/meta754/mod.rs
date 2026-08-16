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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta754(t14055: f64, t9775: f64, t1885: f64, t46722: f64, t13867: f64, t221: f64, t3978: f64, t9921: f64, t14047: f64, t14051: f64, t1412: f64, t5658: f64, t2661: f64, t3938: f64, t3992: f64, t14045: f64, t9810: f64, t13774: f64, t1399: f64, t13927: f64, t48100: f64, t9816: f64, t1353: f64, t13716: f64, t13789: f64, t1410: f64, t3934: f64, t4012: f64, t46660: f64, t48466: f64, t48494: f64, t48498: f64, t48509: f64, t48510: f64, t48514: f64, t5671: f64, t5673: f64, t5674: f64, t828: f64, t9912: f64, t13910: f64, t4057: f64, t5651: f64, t1389: f64, t1882: f64, t46856: f64, t543: f64, t685: f64, t72: f64, t13874: f64, t3989: f64, t13805: f64, t46609: f64, t5608: f64, t4004: f64, t9934: f64, t13854: f64, t9962: f64, t13834: f64, t13999: f64, t125: f64, t13920: f64, t13955: f64, t46946: f64, t13775: f64, t808: f64, t9845: f64, t13783: f64, t13784: f64, t13926: f64, t13944: f64, t36776: f64, t3936: f64, t46671: f64, t46680: f64, t48475: f64, t46917: f64, t5701: f64, t14005: f64, t46740: f64, t5697: f64, t46692: f64, t46695: f64, t46702: f64, t46704: f64, t46706: f64, t46712: f64, t46719: f64, t46723: f64, t46741: f64, t46747: f64, t46749: f64, t5704: f64, t9899: f64, t1872: f64, t9818: f64, t13824: f64, t46716: f64, t13923: f64, t3930: f64, t14036: f64, t9976: f64, t1868: f64, t1883: f64, t46627: f64, t46754: f64, t46757: f64, t46760: f64, t46767: f64, t46771: f64, t46776: f64, t46780: f64, t46787: f64, t46789: f64, t46793: f64, t46797: f64, t9400: f64, t9891: f64, t9984: f64, t46694: f64, t5686: f64, t14030: f64, t9744: f64, t13769: f64, t9736: f64, t13952: f64, t2689: f64, t46825: f64, t9793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48516, t48518, t48527, t48529, t48532, t48533) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2638(t14055, t9775, t1885, t46722, t13867, t221, t3978, t9921, t14047, t14051, t1412, t5658);
        let t48550 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2639(t2661, t3938, t3992, t48533, t14045, t9810, t13774, t1399, t13927, t48100, t9816, t1353, t13716, t13789, t1410, t3934, t4012, t46660, t48466, t48494, t48498, t48509, t48510, t48514, t48516, t48518, t48527, t48529, t48532, t5671, t5673, t5674, t828, t9912);
        let (t48553, t48557, t48563) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2640(t13910, t1399, t2661, t3992, t4057, t5651, t1389, t1882, t46856, t543, t685, t72);
        let (t48565, t48573, t48577, t48591, t48593) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2641(t13874, t3989, t13805, t2661, t46609, t5608, t4004, t9934, t13854, t9962, t13834, t13999);
        let (t48595, t48611) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2642(t125, t13920, t13955, t46946, t13775, t808, t9845, t13783, t13784, t13789, t13926, t13944, t1399, t36776, t3934, t3936, t3938, t46671, t46680, t48475, t48553, t48557, t48563, t48565, t48573, t48577, t48591, t48593, t9810);
        let t48647 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2643(t46917, t5701, t14005, t46740, t5697, t13944, t1399, t3934, t3936, t4004, t4057, t46692, t46695, t46702, t46704, t46706, t46712, t46719, t46723, t46741, t46747, t46749, t48595, t5671, t5673, t5704, t9899);
        let (t48655, t48664, t48666, t48668) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2644(t1872, t4057, t9816, t9818, t13824, t221, t3978, t46716, t13923, t3930, t14036, t9976);
        let t48683 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2645(t48668, t13783, t1410, t1868, t1883, t3934, t3936, t46627, t46754, t46757, t46760, t46767, t46771, t46776, t46780, t46787, t46789, t46793, t46797, t48655, t48664, t48666, t5704, t828, t9400, t9891, t9984);
        let (t48686, t48687, t48691, t48692, t48694, t48696) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2646(t46694, t5686, t14030, t9744, t13769, t808, t9736, t13952, t2689, t13784, t543, t46825, t9793);
    (t48550, t48595, t48611, t48647, t48683, t48686, t48687, t48691, t48692, t48694, t48696)
}
