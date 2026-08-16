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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta758<F: Float>(t13800: F, t46670: F, t3964: F, t5617: F, t9732: F, t136: F, t216: F, t9747: F, t14230: F, t46802: F, t49068: F, t46888: F, t48908: F, t1398: F, t5591: F, t124: F, t1370: F, t13783: F, t13804: F, t221: F, t3889: F, t3934: F, t3936: F, t3938: F, t47296: F, t47298: F, t47302: F, t47304: F, t47306: F, t48421: F, t49062: F, t49066: F, t49071: F, t49085: F, t5627: F, t5671: F, t5704: F, t800: F, t9912: F, t9995: F, t13946: F, t9962: F, t1413: F, t46835: F, t48694: F, t13775: F, t9793: F, t9794: F, t5690: F, t9741: F, t14016: F, t46691: F, t14020: F, t3957: F, t2659: F, t5744: F, t816: F, t13792: F, t48863: F, t13920: F, t2661: F, t3992: F, t543: F, t550: F, t5658: F, t13789: F, t13790: F, t1872: F, t3829: F, t46730: F, t47318: F, t47320: F, t47325: F, t47329: F, t47333: F, t47337: F, t47338: F, t9400: F, t48129: F, t48490: F, t48550: F, t48611: F, t48647: F, t48683: F, t48745: F, t48778: F, t48832: F, t48890: F, t48926: F, t48965: F, t49010: F, t49060: F, t10073: F, t14124: F, t5760: F, t9292: F, t213: F, t46518: F, t46520: F, t46526: F, t48080: F, t48082: F, t48085: F, t48090: F, t546: F, t5735: F, t5755: F, t9899: F, t10069: F, t14207: F, t40921: F, t5737: F, t225: F, t2453: F, t137: F, t14140: F, t2438: F, t4003: F, t10142: F, t14113: F, t10136: F, t14239: F, t10119: F, t4101: F, t5740: F, t9288: F, t1419: F, t2782: F, t4086: F, t40270: F, t14127: F, t14193: F, t22016: F, t4056: F, t5745: F, t9840: F, t555: F, t14122: F, t14171: F, t46433: F, t46536: F, t46540: F, t46542: F, t46561: F, t46563: F, t46568: F, t46570: F, t5675: F, t820: F, t47973: F, t10090: F, t13805: F, t1882: F, t2482: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49087, t49090, t49093, t49103, t49105) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2666::<F>(t13800, t46670, t3964, t5617, t9732, t136, t216, t9747, t14230, t46802, t49068, t46888, t48908);
        let t49112 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2667::<F>(t1398, t5591, t124, t1370, t13783, t13804, t221, t3889, t3934, t3936, t3938, t47296, t47298, t47302, t47304, t47306, t48421, t49062, t49066, t49071, t49085, t49087, t49090, t49093, t49103, t49105, t5627, t5671, t5704, t800, t9912, t9995);
        let (t49118, t49122, t49125, t49127, t49128) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2668::<F>(t13946, t9962, t1413, t46835, t48694, t13775, t9793, t9794, t5690, t9741, t14016, t46691);
        let (t49134, t49139, t49144) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2669::<F>(t14020, t3957, t2659, t5744, t816, t13792, t48863, t13920, t2661, t3992, t543, t550);
        let (t49146, t49157) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2670::<F>(t1398, t5658, t13783, t13789, t13790, t1872, t3829, t3934, t3938, t46730, t47318, t47320, t47325, t47329, t47333, t47337, t47338, t49118, t49122, t49125, t49127, t49128, t49134, t49139, t49144, t5671, t800, t9400);
        let t49161 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2671::<F>(t48129, t48490, t48550, t48611, t48647, t48683, t48745, t48778, t48832, t48890, t48926, t48965, t49010, t49060, t49112, t49157);
        let t49174 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2672::<F>(t10073, t14124, t5760, t9292, t213, t46518, t46520, t46526, t48080, t48082, t48085, t48090, t49161, t546, t5735, t5755, t9899);
        let (t49177, t49178, t49180, t49186) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2673::<F>(t10069, t14207, t40921, t5737, t225, t2453, t136, t137, t1398, t14140, t2438, t4003);
        let (t49187, t49190, t49199, t49200, t49203, t49205) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2674::<F>(t49186, t10142, t14113, t49180, t10136, t14239, t10119, t4101, t5740, t9288, t1419, t5658);
        let t49212 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2675::<F>(t2782, t4086, t49205, t543, t40270, t5737, t14127, t14193, t22016, t4056, t49177, t49178, t49187, t49190, t49199, t49200, t49203, t5735, t5745, t9840);
        let (t49213, t49233) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2676::<F>(t13920, t555, t14122, t14171, t46433, t46536, t46540, t46542, t46561, t46563, t46568, t46570, t5675, t5735, t5745, t5755, t820, t9840, t9912);
        let (t49238, t49242, t49248) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2677::<F>(t10073, t14207, t2782, t4086, t47973, t543, t10090, t13805, t1882, t2482, t686, t72);
    (t49146, t49161, t49174, t49180, t49205, t49212, t49213, t49233, t49238, t49242, t49248)
}
