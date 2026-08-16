//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta757 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2655;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2656;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2657;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2658;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2659;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2660;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2661;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2662;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2663;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2664;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta757(t1412: f64, t808: f64, t13927: f64, t48862: f64, t1389: f64, t14224: f64, t46835: f64, t13769: f64, t2453: f64, t547: f64, t9794: f64, t14230: f64, t2735: f64, t46801: f64, t40763: f64, t5609: f64, t9793: f64, t13830: f64, t9775: f64, t13826: f64, t3989: f64, t1410: f64, t3829: f64, t3934: f64, t46403: f64, t48833: f64, t48837: f64, t48845: f64, t48848: f64, t48849: f64, t48851: f64, t48853: f64, t48855: f64, t5591: f64, t5671: f64, t5673: f64, t5674: f64, t828: f64, t9899: f64, t9942: f64, t13937: f64, t9962: f64, t13991: f64, t13999: f64, t13786: f64, t13760: f64, t9765: f64, t13756: f64, t268: f64, t5617: f64, t46784: f64, t13716: f64, t221: f64, t3978: f64, t3979: f64, t124: f64, t5658: f64, t3938: f64, t9816: f64, t9818: f64, t1414: f64, t46877: f64, t46879: f64, t46885: f64, t46886: f64, t46889: f64, t46893: f64, t46895: f64, t46918: f64, t46922: f64, t48421: f64, t13847: f64, t13848: f64, t4057: f64, t13962: f64, t13845: f64, t5675: f64, t9840: f64, t1889: f64, t46595: f64, t1353: f64, t13767: f64, t2661: f64, t48432: f64, t13768: f64, t3889: f64, t46931: f64, t46934: f64, t46941: f64, t46944: f64, t46947: f64, t46949: f64, t47195: f64, t47199: f64, t47216: f64, t47221: f64, t5689: f64, t800: f64, t9748: f64, t13977: f64, t1399: f64, t13850: f64, t2482: f64, t2668: f64, t4000: f64, t13841: f64, t4010: f64, t13785: f64, t13817: f64, t13981: f64, t13951: f64, t2713: f64, t3964: f64, t14019: f64, t1872: f64, t3944: f64, t47223: f64, t47227: f64, t47229: f64, t47231: f64, t47235: f64, t47239: f64, t47245: f64, t9628: f64, t1413: f64, t48698: f64, t4004: f64, t1873: f64, t46651: f64, t3924: f64, t13910: f64, t9736: f64, t14026: f64, t9744: f64, t125: f64, t13975: f64, t3936: f64, t47259: f64, t47262: f64, t47277: f64, t47282: f64, t47284: f64, t47286: f64, t9891: f64, t13821: f64, t807: f64, t550: f64, t13928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48863, t48865, t48869, t48872, t48876) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2655(t1412, t808, t13927, t48862, t1389, t14224, t46835, t13769, t2453, t547, t9794, t14230, t2735, t46801);
        let t48890 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2656(t48876, t40763, t5609, t9793, t13830, t9775, t13826, t3989, t1410, t3829, t3934, t46403, t48833, t48837, t48845, t48848, t48849, t48851, t48853, t48855, t48865, t48869, t48872, t5591, t5671, t5673, t5674, t828, t9899, t9942);
        let (t48892, t48900, t48902, t48905, t48906, t48908, t48909) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2657(t13937, t9962, t13991, t13999, t13786, t13760, t9765, t13756, t3989, t268, t5617, t46784);
        let (t48919, t48926) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2658(t13716, t221, t3978, t3979, t124, t5658, t3938, t9816, t9818, t1410, t1414, t46877, t46879, t46885, t46886, t46889, t46893, t46895, t46918, t46922, t48421, t48892, t48900, t48902, t48905, t48906, t48909, t828);
        let (t48929, t48937, t48941, t48945, t48947) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2659(t13847, t13848, t4057, t9816, t13962, t9962, t13845, t48919, t5675, t9840, t1889, t46595);
        let t48965 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2660(t1353, t13767, t2661, t48432, t13768, t3889, t3829, t46931, t46934, t46941, t46944, t46947, t46949, t47195, t47199, t47216, t47221, t48929, t48937, t48941, t48945, t48947, t5689, t800, t9748);
        let (t48971, t48975, t48982, t48984, t48999) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2661(t13977, t9962, t13847, t1399, t48919, t9816, t13850, t2482, t2668, t4000, t13841, t4010, t808);
        let t49010 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2662(t13785, t48862, t48999, t13817, t13999, t13981, t9962, t13951, t2713, t3964, t1353, t14019, t1872, t3889, t3944, t47223, t47227, t47229, t47231, t47235, t47239, t47245, t48971, t48975, t48982, t48984, t5689, t800, t9628);
        let (t49012, t49016, t49024, t49030, t49049) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2663(t1413, t46835, t48698, t13845, t13847, t13848, t4004, t1872, t9818, t1873, t46651, t1399, t5689, t9816);
        let t49060 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2664(t13847, t13848, t3924, t9816, t13910, t808, t9736, t14026, t9744, t125, t13716, t13975, t1399, t3934, t3936, t4004, t4057, t47259, t47262, t47277, t47282, t47284, t47286, t49012, t49016, t49024, t49030, t49049, t5671, t5673, t5674, t9891);
        let (t49062, t49066, t49068, t49071, t49085) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2665(t13821, t13999, t13716, t1413, t547, t807, t550, t9794, t14224, t9793, t13928, t9962);
    (t48863, t48890, t48908, t48926, t48965, t49010, t49060, t49062, t49066, t49068, t49071, t49085)
}
