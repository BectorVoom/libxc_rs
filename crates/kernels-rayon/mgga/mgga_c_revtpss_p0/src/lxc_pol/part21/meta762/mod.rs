//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta762 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2702;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2703;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2704;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2705;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2706;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2707;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta762(t116: f64, t13424: f64, t2371: f64, t648: f64, t10199: f64, t1514: f64, t2289: f64, t4264: f64, t13459: f64, t625: f64, t13462: f64, t13510: f64, t105: f64, t4283: f64, t588: f64, t100: f64, t10217: f64, t10236: f64, t10243: f64, t10247: f64, t10250: f64, t10251: f64, t108: f64, t13479: f64, t13482: f64, t1505: f64, t1507: f64, t22: f64, t2344: f64, t2357: f64, t4269: f64, t4270: f64, t4274: f64, t4279: f64, t580: f64, t656: f64, t661: f64, t97: f64, t2349: f64, t10227: f64, t10241: f64, t4273: f64, t10228: f64, t10242: f64, t13472: f64, t13475: f64, t13476: f64, t13485: f64, t13496: f64, t1504: f64, t1509: f64, t2255: f64, t2256: f64, t2350: f64, t2358: f64, t2362: f64, t31283: f64, t31443: f64, t46196: f64, t46212: f64, t658: f64, t9342: f64, t4288: f64, t13455: f64, t10209: f64, t1513: f64, t2366: f64, t28036: f64, t31035: f64, t46146: f64, t46148: f64, t46150: f64, t46152: f64, t46154: f64, t46157: f64, t655: f64, t69: f64, t114: f64, t10208: f64, t10254: f64, t13458: f64, t13509: f64, t2339: f64, t2340: f64, t4263: f64, t4287: f64, t46143: f64, t46144: f64, t665: f64, t10179: f64, t10192: f64, t10416: f64, t1310: f64, t1315: f64, t13426: f64, t13429: f64, t13435: f64, t13521: f64, t1353: f64, t13600: f64, t13625: f64, t13648: f64, t13674: f64, t13867: f64, t14310: f64, t1450: f64, t1519: f64, t18153: f64, t1847: f64, t1868: f64, t1907: f64, t198: f64, t2331: f64, t3829: f64, t3889: f64, t39483: f64, t39520: f64, t39747: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39807: f64, t39813: f64, t40067: f64, t40072: f64, t40076: f64, t4135: f64, t4139: f64, t4140: f64, t4151: f64, t4254: f64, t4257: f64, t4293: f64, t46970: f64, t46988: f64, t47063: f64, t47067: f64, t47070: f64, t47072: f64, t47084: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47651: f64, t48219: f64, t48223: f64, t48224: f64, t48226: f64, t48241: f64, t48243: f64, t48244: f64, t48247: f64, t48248: f64, t48249: f64, t48250: f64, t48251: f64, t48258: f64, t48259: f64, t48261: f64, t48263: f64, t48264: f64, t48278: f64, t48279: f64, t48296: f64, t48298: f64, t48300: f64, t48303: f64, t48312: f64, t48314: f64, t48322: f64, t48323: f64, t48325: f64, t48327: f64, t49550: f64, t49571: f64, t49579: f64, t49592: f64, t49611: f64, t49616: f64, t49634: f64, t49640: f64, t49647: f64, t49654: f64, t49659: f64, t49675: f64, t508: f64, t511: f64, t5528: f64, t5532: f64, t5536: f64, t5541: f64, t5542: f64, t5591: f64, t5627: f64, t649: f64, t651: f64, t671: f64, t9400: f64, t9547: f64, t9599: f64, t9628: f64, t9984: f64, t2319: f64, t670: f64, t10259: f64, t94: f64, t14619: f64, t750: f64, t4398: f64, t9372: f64, t39423: f64, t39425: f64, t39433: f64, t39436: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49686, t49693, t49698, t49701, t49702, t49704, t49724) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2702(t116, t13424, t2371, t648, t10199, t1514, t2289, t4264, t13459, t625, t13462, t13510);
        let t49760 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2703(t105, t4283, t588, t100, t10217, t10236, t10243, t10247, t10250, t10251, t108, t13479, t13482, t1505, t1507, t22, t2344, t2357, t4269, t4270, t4274, t4279, t580, t656, t661, t97);
        let t49809 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2704(t2349, t656, t10227, t97, t10241, t105, t4273, t588, t10228, t10242, t13472, t13475, t13476, t13485, t13496, t1504, t1509, t2255, t2256, t2350, t2358, t2362, t31283, t31443, t46196, t46212, t580, t658, t661, t9342);
        let t49828 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2705(t2289, t4288, t13455, t625, t10209, t1513, t2366, t28036, t31035, t46146, t46148, t46150, t46152, t46154, t46157, t49724, t49760, t49809, t655, t69);
        let t49830 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2706(t114, t10208, t10254, t13458, t13509, t2339, t2340, t2366, t4263, t4287, t46143, t46144, t49698, t49701, t49702, t49704, t49828, t665, t69);
        let t49834 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2707(t10179, t10192, t10416, t1310, t1315, t13426, t13429, t13435, t13521, t1353, t13600, t13625, t13648, t13674, t13867, t14310, t1450, t1519, t18153, t1847, t1868, t1907, t198, t2331, t3829, t3889, t39483, t39520, t39747, t39773, t39783, t39786, t39791, t39795, t39807, t39813, t40067, t40072, t40076, t4135, t4139, t4140, t4151, t4254, t4257, t4293, t46970, t46988, t47063, t47067, t47070, t47072, t47084, t47131, t47138, t47140, t47142, t47651, t48219, t48223, t48224, t48226, t48241, t48243, t48244, t48247, t48248, t48249, t48250, t48251, t48258, t48259, t48261, t48263, t48264, t48278, t48279, t48296, t48298, t48300, t48303, t48312, t48314, t48322, t48323, t48325, t48327, t49550, t49571, t49579, t49592, t49611, t49616, t49634, t49640, t49647, t49654, t49659, t49675, t49686, t49693, t49830, t508, t511, t5528, t5532, t5536, t5541, t5542, t5591, t5627, t649, t651, t671, t9400, t9547, t9599, t9628, t9984);
        let (t49851, t49856, t49865, t49867, t49868, t49869, t49870, t49872) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2708(t2319, t670, t10259, t94, t14619, t750, t4398, t9372, t39423, t39425, t39433, t39436);
    (t49686, t49693, t49830, t49834, t49851, t49856, t49865, t49867, t49868, t49869, t49870, t49872)
}
