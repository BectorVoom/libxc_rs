//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta762 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2702;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2703;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2704;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2705;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2706;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2707;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta762<F: Float>(t116: F, t13424: F, t2371: F, t648: F, t10199: F, t1514: F, t2289: F, t4264: F, t13459: F, t625: F, t13462: F, t13510: F, t105: F, t4283: F, t588: F, t100: F, t10217: F, t10236: F, t10243: F, t10247: F, t10250: F, t10251: F, t108: F, t13479: F, t13482: F, t1505: F, t1507: F, t22: F, t2344: F, t2357: F, t4269: F, t4270: F, t4274: F, t4279: F, t580: F, t656: F, t661: F, t97: F, t2349: F, t10227: F, t10241: F, t4273: F, t10228: F, t10242: F, t13472: F, t13475: F, t13476: F, t13485: F, t13496: F, t1504: F, t1509: F, t2255: F, t2256: F, t2350: F, t2358: F, t2362: F, t31283: F, t31443: F, t46196: F, t46212: F, t658: F, t9342: F, t4288: F, t13455: F, t10209: F, t1513: F, t2366: F, t28036: F, t31035: F, t46146: F, t46148: F, t46150: F, t46152: F, t46154: F, t46157: F, t655: F, t69: F, t114: F, t10208: F, t10254: F, t13458: F, t13509: F, t2339: F, t2340: F, t4263: F, t4287: F, t46143: F, t46144: F, t665: F, t10179: F, t10192: F, t10416: F, t1310: F, t1315: F, t13426: F, t13429: F, t13435: F, t13521: F, t1353: F, t13600: F, t13625: F, t13648: F, t13674: F, t13867: F, t14310: F, t1450: F, t1519: F, t18153: F, t1847: F, t1868: F, t1907: F, t198: F, t2331: F, t3829: F, t3889: F, t39483: F, t39520: F, t39747: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39807: F, t39813: F, t40067: F, t40072: F, t40076: F, t4135: F, t4139: F, t4140: F, t4151: F, t4254: F, t4257: F, t4293: F, t46970: F, t46988: F, t47063: F, t47067: F, t47070: F, t47072: F, t47084: F, t47131: F, t47138: F, t47140: F, t47142: F, t47651: F, t48219: F, t48223: F, t48224: F, t48226: F, t48241: F, t48243: F, t48244: F, t48247: F, t48248: F, t48249: F, t48250: F, t48251: F, t48258: F, t48259: F, t48261: F, t48263: F, t48264: F, t48278: F, t48279: F, t48296: F, t48298: F, t48300: F, t48303: F, t48312: F, t48314: F, t48322: F, t48323: F, t48325: F, t48327: F, t49550: F, t49571: F, t49579: F, t49592: F, t49611: F, t49616: F, t49634: F, t49640: F, t49647: F, t49654: F, t49659: F, t49675: F, t508: F, t511: F, t5528: F, t5532: F, t5536: F, t5541: F, t5542: F, t5591: F, t5627: F, t649: F, t651: F, t671: F, t9400: F, t9547: F, t9599: F, t9628: F, t9984: F, t2319: F, t670: F, t10259: F, t94: F, t14619: F, t750: F, t4398: F, t9372: F, t39423: F, t39425: F, t39433: F, t39436: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49686, t49693, t49698, t49701, t49702, t49704, t49724) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2702::<F>(t116, t13424, t2371, t648, t10199, t1514, t2289, t4264, t13459, t625, t13462, t13510);
        let t49760 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2703::<F>(t105, t4283, t588, t100, t10217, t10236, t10243, t10247, t10250, t10251, t108, t13479, t13482, t1505, t1507, t22, t2344, t2357, t4269, t4270, t4274, t4279, t580, t656, t661, t97);
        let t49809 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2704::<F>(t2349, t656, t10227, t97, t10241, t105, t4273, t588, t10228, t10242, t13472, t13475, t13476, t13485, t13496, t1504, t1509, t2255, t2256, t2350, t2358, t2362, t31283, t31443, t46196, t46212, t580, t658, t661, t9342);
        let t49828 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2705::<F>(t2289, t4288, t13455, t625, t10209, t1513, t2366, t28036, t31035, t46146, t46148, t46150, t46152, t46154, t46157, t49724, t49760, t49809, t655, t69);
        let t49830 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2706::<F>(t114, t10208, t10254, t13458, t13509, t2339, t2340, t2366, t4263, t4287, t46143, t46144, t49698, t49701, t49702, t49704, t49828, t665, t69);
        let t49834 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2707::<F>(t10179, t10192, t10416, t1310, t1315, t13426, t13429, t13435, t13521, t1353, t13600, t13625, t13648, t13674, t13867, t14310, t1450, t1519, t18153, t1847, t1868, t1907, t198, t2331, t3829, t3889, t39483, t39520, t39747, t39773, t39783, t39786, t39791, t39795, t39807, t39813, t40067, t40072, t40076, t4135, t4139, t4140, t4151, t4254, t4257, t4293, t46970, t46988, t47063, t47067, t47070, t47072, t47084, t47131, t47138, t47140, t47142, t47651, t48219, t48223, t48224, t48226, t48241, t48243, t48244, t48247, t48248, t48249, t48250, t48251, t48258, t48259, t48261, t48263, t48264, t48278, t48279, t48296, t48298, t48300, t48303, t48312, t48314, t48322, t48323, t48325, t48327, t49550, t49571, t49579, t49592, t49611, t49616, t49634, t49640, t49647, t49654, t49659, t49675, t49686, t49693, t49830, t508, t511, t5528, t5532, t5536, t5541, t5542, t5591, t5627, t649, t651, t671, t9400, t9547, t9599, t9628, t9984);
        let (t49851, t49856, t49865, t49867, t49868, t49869, t49870, t49872) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2708::<F>(t2319, t670, t10259, t94, t14619, t750, t4398, t9372, t39423, t39425, t39433, t39436);
    (t49686, t49693, t49830, t49834, t49851, t49856, t49865, t49867, t49868, t49869, t49870, t49872)
}
