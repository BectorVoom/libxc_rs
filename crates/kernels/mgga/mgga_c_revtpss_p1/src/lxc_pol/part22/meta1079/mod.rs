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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1079<F: Float>(t1413: F, t21969: F, t547: F, t807: F, t13789: F, t13790: F, t1410: F, t3829: F, t46627: F, t46828: F, t46831: F, t46833: F, t46837: F, t46840: F, t46859: F, t46861: F, t48756: F, t5671: F, t6836: F, t73837: F, t828: F, t221: F, t22274: F, t3978: F, t46716: F, t22279: F, t9921: F, t22255: F, t3930: F, t22259: F, t9976: F, t22125: F, t2713: F, t3964: F, t48786: F, t48790: F, t48792: F, t48794: F, t48796: F, t48811: F, t48813: F, t3889: F, t3944: F, t48825: F, t48827: F, t48829: F, t48833: F, t48837: F, t48845: F, t48847: F, t48849: F, t48851: F, t48853: F, t6883: F, t800: F, t13848: F, t22096: F, t9816: F, t9818: F, t13845: F, t13847: F, t5675: F, t73856: F, t22107: F, t9962: F, t1399: F, t22245: F, t2661: F, t3992: F, t22287: F, t22289: F, t3989: F, t1868: F, t1883: F, t46825: F, t9793: F, t47274: F, t6849: F, t22126: F, t2689: F, t22130: F, t13867: F, t47248: F, t48712: F, t48855: F, t5704: F, t22081: F, t22276: F, t22281: F, t22056: F, t9765: F, t48865: F, t48868: F, t48872: F, t48876: F, t48879: F, t48881: F, t48888: F, t22021: F, t808: F, t9845: F, t46879: F, t46885: F, t46886: F, t46889: F, t46895: F, t48892: F, t48900: F, t48902: F, t48904: F, t48906: F, t48909: F, t46918: F, t46931: F, t46934: F, t46941: F, t46944: F, t46947: F, t48917: F, t48922: F, t48929: F, t48937: F, t48941: F, t22041: F, t3957: F, t124: F, t1370: F, t47199: F, t47216: F, t47229: F, t48945: F, t48947: F, t48951: F, t48955: F, t48971: F, t48975: F, t73578: F, t22074: F, t3936: F, t4004: F, t48982: F, t48984: F, t49001: F, t49003: F, t49005: F, t49008: F, t49012: F, t49016: F, t49024: F, t49030: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t74418 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3870::<F>(t1413, t21969, t547, t807, t13789, t13790, t1410, t3829, t46627, t46828, t46831, t46833, t46837, t46840, t46859, t46861, t48756, t5671, t6836, t73837, t828);
        let (t74421, t74425, t74427, t74429, t74437) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3871::<F>(t221, t22274, t3978, t46716, t22279, t9921, t22255, t3930, t22259, t9976, t22125, t2713, t3964);
        let t74441 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3872::<F>(t48786, t48790, t48792, t48794, t48796, t48811, t48813, t74421, t74425, t74427, t74429, t74437);
        let t74458 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3873::<F>(t3889, t3944, t48825, t48827, t48829, t48833, t48837, t48845, t48847, t48849, t48851, t48853, t6883, t800);
        let (t74461, t74469, t74471, t74475) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3874::<F>(t13848, t22096, t9816, t9818, t13845, t13847, t5675, t73856, t22107, t9962, t1399, t22245, t2661, t3992);
        let (t74479, t74481, t74483, t74485, t74489) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3875::<F>(t221, t22287, t3978, t9921, t22289, t3989, t1868, t1883, t46825, t9793, t1399, t47274, t6849, t9816);
        let t74496 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3876::<F>(t22126, t2689, t22130, t13867, t47248, t48712, t48855, t5704, t74461, t74469, t74471, t74475, t74479, t74481, t74485, t74489);
        let t74513 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3877::<F>(t22081, t9962, t22276, t3989, t22281, t22056, t9765, t48865, t48868, t48872, t48876, t48879, t48881, t48888);
        let t74527 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3878::<F>(t22021, t808, t9845, t46879, t46885, t46886, t46889, t46895, t48892, t48900, t48902, t48904, t48906, t48909);
        let (t74542, t74547) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3879::<F>(t46918, t46931, t46934, t46941, t46944, t46947, t48917, t48922, t48929, t48937, t48941, t22041, t3957);
        let t74558 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3880::<F>(t124, t1370, t47199, t47216, t47229, t48945, t48947, t48951, t48955, t48971, t48975, t73578, t74547, t800);
        let t74574 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3881::<F>(t22074, t3936, t4004, t48982, t48984, t49001, t49003, t49005, t49008, t49012, t49016, t49024, t49030, t5671);
    (t74418, t74441, t74458, t74483, t74496, t74513, t74527, t74542, t74558, t74574)
}
