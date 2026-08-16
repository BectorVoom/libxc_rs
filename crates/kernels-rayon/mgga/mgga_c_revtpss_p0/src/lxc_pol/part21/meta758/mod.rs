//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta758 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2666;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2667;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2668;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2669;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2670;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2671;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2672;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2673;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2674;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2675;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2676;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta758(t13800: f64, t46670: f64, t3964: f64, t5617: f64, t9732: f64, t136: f64, t216: f64, t9747: f64, t14230: f64, t46802: f64, t49068: f64, t46888: f64, t48908: f64, t1398: f64, t5591: f64, t124: f64, t1370: f64, t13783: f64, t13804: f64, t221: f64, t3889: f64, t3934: f64, t3936: f64, t3938: f64, t47296: f64, t47298: f64, t47302: f64, t47304: f64, t47306: f64, t48421: f64, t49062: f64, t49066: f64, t49071: f64, t49085: f64, t5627: f64, t5671: f64, t5704: f64, t800: f64, t9912: f64, t9995: f64, t13946: f64, t9962: f64, t1413: f64, t46835: f64, t48694: f64, t13775: f64, t9793: f64, t9794: f64, t5690: f64, t9741: f64, t14016: f64, t46691: f64, t14020: f64, t3957: f64, t2659: f64, t5744: f64, t816: f64, t13792: f64, t48863: f64, t13920: f64, t2661: f64, t3992: f64, t543: f64, t550: f64, t5658: f64, t13789: f64, t13790: f64, t1872: f64, t3829: f64, t46730: f64, t47318: f64, t47320: f64, t47325: f64, t47329: f64, t47333: f64, t47337: f64, t47338: f64, t9400: f64, t48129: f64, t48490: f64, t48550: f64, t48611: f64, t48647: f64, t48683: f64, t48745: f64, t48778: f64, t48832: f64, t48890: f64, t48926: f64, t48965: f64, t49010: f64, t49060: f64, t10073: f64, t14124: f64, t5760: f64, t9292: f64, t213: f64, t46518: f64, t46520: f64, t46526: f64, t48080: f64, t48082: f64, t48085: f64, t48090: f64, t546: f64, t5735: f64, t5755: f64, t9899: f64, t10069: f64, t14207: f64, t40921: f64, t5737: f64, t225: f64, t2453: f64, t137: f64, t14140: f64, t2438: f64, t4003: f64, t10142: f64, t14113: f64, t10136: f64, t14239: f64, t10119: f64, t4101: f64, t5740: f64, t9288: f64, t1419: f64, t2782: f64, t4086: f64, t40270: f64, t14127: f64, t14193: f64, t22016: f64, t4056: f64, t5745: f64, t9840: f64, t555: f64, t14122: f64, t14171: f64, t46433: f64, t46536: f64, t46540: f64, t46542: f64, t46561: f64, t46563: f64, t46568: f64, t46570: f64, t5675: f64, t820: f64, t47973: f64, t10090: f64, t13805: f64, t1882: f64, t2482: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49087, t49090, t49093, t49103, t49105) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2666(t13800, t46670, t3964, t5617, t9732, t136, t216, t9747, t14230, t46802, t49068, t46888, t48908);
        let t49112 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2667(t1398, t5591, t124, t1370, t13783, t13804, t221, t3889, t3934, t3936, t3938, t47296, t47298, t47302, t47304, t47306, t48421, t49062, t49066, t49071, t49085, t49087, t49090, t49093, t49103, t49105, t5627, t5671, t5704, t800, t9912, t9995);
        let (t49118, t49122, t49125, t49127, t49128) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2668(t13946, t9962, t1413, t46835, t48694, t13775, t9793, t9794, t5690, t9741, t14016, t46691);
        let (t49134, t49139, t49144) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2669(t14020, t3957, t2659, t5744, t816, t13792, t48863, t13920, t2661, t3992, t543, t550);
        let (t49146, t49157) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2670(t1398, t5658, t13783, t13789, t13790, t1872, t3829, t3934, t3938, t46730, t47318, t47320, t47325, t47329, t47333, t47337, t47338, t49118, t49122, t49125, t49127, t49128, t49134, t49139, t49144, t5671, t800, t9400);
        let t49161 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2671(t48129, t48490, t48550, t48611, t48647, t48683, t48745, t48778, t48832, t48890, t48926, t48965, t49010, t49060, t49112, t49157);
        let t49174 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2672(t10073, t14124, t5760, t9292, t213, t46518, t46520, t46526, t48080, t48082, t48085, t48090, t49161, t546, t5735, t5755, t9899);
        let (t49177, t49178, t49180, t49186) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2673(t10069, t14207, t40921, t5737, t225, t2453, t136, t137, t1398, t14140, t2438, t4003);
        let (t49187, t49190, t49199, t49200, t49203, t49205) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2674(t49186, t10142, t14113, t49180, t10136, t14239, t10119, t4101, t5740, t9288, t1419, t5658);
        let t49212 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2675(t2782, t4086, t49205, t543, t40270, t5737, t14127, t14193, t22016, t4056, t49177, t49178, t49187, t49190, t49199, t49200, t49203, t5735, t5745, t9840);
        let (t49213, t49233) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2676(t13920, t555, t14122, t14171, t46433, t46536, t46540, t46542, t46561, t46563, t46568, t46570, t5675, t5735, t5745, t5755, t820, t9840, t9912);
        let (t49238, t49242, t49248) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2677(t10073, t14207, t2782, t4086, t47973, t543, t10090, t13805, t1882, t2482, t686, t72);
    (t49146, t49161, t49174, t49180, t49205, t49212, t49213, t49233, t49238, t49242, t49248)
}
