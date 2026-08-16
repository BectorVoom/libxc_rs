//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1079 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3870;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3871;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3872;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3873;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3874;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3875;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3876;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3877;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3878;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3879;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3880;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1079(t1413: f64, t21969: f64, t547: f64, t807: f64, t13789: f64, t13790: f64, t1410: f64, t3829: f64, t46627: f64, t46828: f64, t46831: f64, t46833: f64, t46837: f64, t46840: f64, t46859: f64, t46861: f64, t48756: f64, t5671: f64, t6836: f64, t73837: f64, t828: f64, t221: f64, t22274: f64, t3978: f64, t46716: f64, t22279: f64, t9921: f64, t22255: f64, t3930: f64, t22259: f64, t9976: f64, t22125: f64, t2713: f64, t3964: f64, t48786: f64, t48790: f64, t48792: f64, t48794: f64, t48796: f64, t48811: f64, t48813: f64, t3889: f64, t3944: f64, t48825: f64, t48827: f64, t48829: f64, t48833: f64, t48837: f64, t48845: f64, t48847: f64, t48849: f64, t48851: f64, t48853: f64, t6883: f64, t800: f64, t13848: f64, t22096: f64, t9816: f64, t9818: f64, t13845: f64, t13847: f64, t5675: f64, t73856: f64, t22107: f64, t9962: f64, t1399: f64, t22245: f64, t2661: f64, t3992: f64, t22287: f64, t22289: f64, t3989: f64, t1868: f64, t1883: f64, t46825: f64, t9793: f64, t47274: f64, t6849: f64, t22126: f64, t2689: f64, t22130: f64, t13867: f64, t47248: f64, t48712: f64, t48855: f64, t5704: f64, t22081: f64, t22276: f64, t22281: f64, t22056: f64, t9765: f64, t48865: f64, t48868: f64, t48872: f64, t48876: f64, t48879: f64, t48881: f64, t48888: f64, t22021: f64, t808: f64, t9845: f64, t46879: f64, t46885: f64, t46886: f64, t46889: f64, t46895: f64, t48892: f64, t48900: f64, t48902: f64, t48904: f64, t48906: f64, t48909: f64, t46918: f64, t46931: f64, t46934: f64, t46941: f64, t46944: f64, t46947: f64, t48917: f64, t48922: f64, t48929: f64, t48937: f64, t48941: f64, t22041: f64, t3957: f64, t124: f64, t1370: f64, t47199: f64, t47216: f64, t47229: f64, t48945: f64, t48947: f64, t48951: f64, t48955: f64, t48971: f64, t48975: f64, t73578: f64, t22074: f64, t3936: f64, t4004: f64, t48982: f64, t48984: f64, t49001: f64, t49003: f64, t49005: f64, t49008: f64, t49012: f64, t49016: f64, t49024: f64, t49030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t74418 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3870(t1413, t21969, t547, t807, t13789, t13790, t1410, t3829, t46627, t46828, t46831, t46833, t46837, t46840, t46859, t46861, t48756, t5671, t6836, t73837, t828);
        let (t74421, t74425, t74427, t74429, t74437) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3871(t221, t22274, t3978, t46716, t22279, t9921, t22255, t3930, t22259, t9976, t22125, t2713, t3964);
        let t74441 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3872(t48786, t48790, t48792, t48794, t48796, t48811, t48813, t74421, t74425, t74427, t74429, t74437);
        let t74458 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3873(t3889, t3944, t48825, t48827, t48829, t48833, t48837, t48845, t48847, t48849, t48851, t48853, t6883, t800);
        let (t74461, t74469, t74471, t74475) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3874(t13848, t22096, t9816, t9818, t13845, t13847, t5675, t73856, t22107, t9962, t1399, t22245, t2661, t3992);
        let (t74479, t74481, t74483, t74485, t74489) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3875(t221, t22287, t3978, t9921, t22289, t3989, t1868, t1883, t46825, t9793, t1399, t47274, t6849, t9816);
        let t74496 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3876(t22126, t2689, t22130, t13867, t47248, t48712, t48855, t5704, t74461, t74469, t74471, t74475, t74479, t74481, t74485, t74489);
        let t74513 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3877(t22081, t9962, t22276, t3989, t22281, t22056, t9765, t48865, t48868, t48872, t48876, t48879, t48881, t48888);
        let t74527 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3878(t22021, t808, t9845, t46879, t46885, t46886, t46889, t46895, t48892, t48900, t48902, t48904, t48906, t48909);
        let (t74542, t74547) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3879(t46918, t46931, t46934, t46941, t46944, t46947, t48917, t48922, t48929, t48937, t48941, t22041, t3957);
        let t74558 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3880(t124, t1370, t47199, t47216, t47229, t48945, t48947, t48951, t48955, t48971, t48975, t73578, t74547, t800);
        let t74574 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3881(t22074, t3936, t4004, t48982, t48984, t49001, t49003, t49005, t49008, t49012, t49016, t49024, t49030, t5671);
    (t74418, t74441, t74458, t74483, t74496, t74513, t74527, t74542, t74558, t74574)
}
