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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta757<F: Float>(t1412: F, t808: F, t13927: F, t48862: F, t1389: F, t14224: F, t46835: F, t13769: F, t2453: F, t547: F, t9794: F, t14230: F, t2735: F, t46801: F, t40763: F, t5609: F, t9793: F, t13830: F, t9775: F, t13826: F, t3989: F, t1410: F, t3829: F, t3934: F, t46403: F, t48833: F, t48837: F, t48845: F, t48848: F, t48849: F, t48851: F, t48853: F, t48855: F, t5591: F, t5671: F, t5673: F, t5674: F, t828: F, t9899: F, t9942: F, t13937: F, t9962: F, t13991: F, t13999: F, t13786: F, t13760: F, t9765: F, t13756: F, t268: F, t5617: F, t46784: F, t13716: F, t221: F, t3978: F, t3979: F, t124: F, t5658: F, t3938: F, t9816: F, t9818: F, t1414: F, t46877: F, t46879: F, t46885: F, t46886: F, t46889: F, t46893: F, t46895: F, t46918: F, t46922: F, t48421: F, t13847: F, t13848: F, t4057: F, t13962: F, t13845: F, t5675: F, t9840: F, t1889: F, t46595: F, t1353: F, t13767: F, t2661: F, t48432: F, t13768: F, t3889: F, t46931: F, t46934: F, t46941: F, t46944: F, t46947: F, t46949: F, t47195: F, t47199: F, t47216: F, t47221: F, t5689: F, t800: F, t9748: F, t13977: F, t1399: F, t13850: F, t2482: F, t2668: F, t4000: F, t13841: F, t4010: F, t13785: F, t13817: F, t13981: F, t13951: F, t2713: F, t3964: F, t14019: F, t1872: F, t3944: F, t47223: F, t47227: F, t47229: F, t47231: F, t47235: F, t47239: F, t47245: F, t9628: F, t1413: F, t48698: F, t4004: F, t1873: F, t46651: F, t3924: F, t13910: F, t9736: F, t14026: F, t9744: F, t125: F, t13975: F, t3936: F, t47259: F, t47262: F, t47277: F, t47282: F, t47284: F, t47286: F, t9891: F, t13821: F, t807: F, t550: F, t13928: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48863, t48865, t48869, t48872, t48876) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2655::<F>(t1412, t808, t13927, t48862, t1389, t14224, t46835, t13769, t2453, t547, t9794, t14230, t2735, t46801);
        let t48890 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2656::<F>(t48876, t40763, t5609, t9793, t13830, t9775, t13826, t3989, t1410, t3829, t3934, t46403, t48833, t48837, t48845, t48848, t48849, t48851, t48853, t48855, t48865, t48869, t48872, t5591, t5671, t5673, t5674, t828, t9899, t9942);
        let (t48892, t48900, t48902, t48905, t48906, t48908, t48909) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2657::<F>(t13937, t9962, t13991, t13999, t13786, t13760, t9765, t13756, t3989, t268, t5617, t46784);
        let (t48919, t48926) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2658::<F>(t13716, t221, t3978, t3979, t124, t5658, t3938, t9816, t9818, t1410, t1414, t46877, t46879, t46885, t46886, t46889, t46893, t46895, t46918, t46922, t48421, t48892, t48900, t48902, t48905, t48906, t48909, t828);
        let (t48929, t48937, t48941, t48945, t48947) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2659::<F>(t13847, t13848, t4057, t9816, t13962, t9962, t13845, t48919, t5675, t9840, t1889, t46595);
        let t48965 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2660::<F>(t1353, t13767, t2661, t48432, t13768, t3889, t3829, t46931, t46934, t46941, t46944, t46947, t46949, t47195, t47199, t47216, t47221, t48929, t48937, t48941, t48945, t48947, t5689, t800, t9748);
        let (t48971, t48975, t48982, t48984, t48999) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2661::<F>(t13977, t9962, t13847, t1399, t48919, t9816, t13850, t2482, t2668, t4000, t13841, t4010, t808);
        let t49010 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2662::<F>(t13785, t48862, t48999, t13817, t13999, t13981, t9962, t13951, t2713, t3964, t1353, t14019, t1872, t3889, t3944, t47223, t47227, t47229, t47231, t47235, t47239, t47245, t48971, t48975, t48982, t48984, t5689, t800, t9628);
        let (t49012, t49016, t49024, t49030, t49049) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2663::<F>(t1413, t46835, t48698, t13845, t13847, t13848, t4004, t1872, t9818, t1873, t46651, t1399, t5689, t9816);
        let t49060 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2664::<F>(t13847, t13848, t3924, t9816, t13910, t808, t9736, t14026, t9744, t125, t13716, t13975, t1399, t3934, t3936, t4004, t4057, t47259, t47262, t47277, t47282, t47284, t47286, t49012, t49016, t49024, t49030, t49049, t5671, t5673, t5674, t9891);
        let (t49062, t49066, t49068, t49071, t49085) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2665::<F>(t13821, t13999, t13716, t1413, t547, t807, t550, t9794, t14224, t9793, t13928, t9962);
    (t48863, t48890, t48908, t48926, t48965, t49010, t49060, t49062, t49066, t49068, t49071, t49085)
}
