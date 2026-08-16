//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta760 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2684;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2685;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2686;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2687;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2688;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2689;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2690;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2691;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2692;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2693;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2694;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta760<F: Float>(t1897: F, t40317: F, t10111: F, t22: F, t5759: F, t49146: F, t543: F, t2782: F, t4100: F, t48475: F, t47423: F, t5741: F, t3923: F, t48105: F, t47371: F, t1399: F, t14122: F, t4057: F, t47424: F, t47427: F, t47432: F, t47436: F, t49205: F, t5735: F, t5755: F, t9891: F, t10026: F, t14141: F, t14143: F, t4056: F, t676: F, t14066: F, t1432: F, t686: F, t72: F, t14188: F, t2439: F, t2777: F, t10130: F, t1437: F, t4004: F, t47442: F, t47444: F, t48438: F, t49280: F, t5659: F, t5745: F, t5767: F, t820: F, t9899: F, t10073: F, t14129: F, t14159: F, t3964: F, t9285: F, t213: F, t225: F, t46475: F, t10019: F, t14114: F, t14145: F, t2482: F, t4114: F, t5658: F, t14193: F, t46422: F, t47450: F, t47454: F, t47455: F, t48760: F, t49213: F, t9995: F, t10171: F, t13921: F, t14127: F, t1424: F, t1427: F, t1904: F, t4118: F, t46350: F, t46392: F, t46398: F, t46401: F, t46403: F, t46412: F, t46424: F, t46435: F, t46496: F, t46500: F, t46505: F, t46510: F, t46515: F, t46572: F, t46583: F, t46587: F, t47403: F, t47407: F, t47411: F, t47413: F, t47417: F, t47550: F, t47554: F, t47558: F, t47561: F, t47926: F, t47929: F, t47932: F, t47936: F, t47938: F, t47942: F, t47945: F, t47948: F, t47953: F, t47957: F, t47961: F, t47964: F, t47967: F, t47992: F, t48024: F, t48052: F, t48058: F, t48066: F, t48076: F, t49174: F, t49212: F, t49233: F, t49238: F, t49242: F, t49248: F, t49252: F, t49256: F, t49260: F, t49263: F, t49293: F, t49310: F, t49348: F, t5675: F, t5775: F, t9912: F, t5600: F, t9292: F, t1893: F, t4075: F, t786: F, t9682: F, t10115: F, t1894: F, t14094: F, t2435: F, t1358: F, t5710: F, t785: F, t4077: F, t47794: F, t556: F, t1426: F, t5711: F, t3917: F, t14269: F, t14299: F, t4071: F, t4132: F, t47568: F, t47570: F, t47574: F, t47580: F, t47591: F, t49161: F, t561: F, t5774: F, t9657: F, t3899: F, t689: F, t14100: F, t9686: F, t13729: F, t4131: F, t47506: F, t5722: F, t14268: F, t1444: F, t4076: F, t47593: F, t47595: F, t47601: F, t47606: F, t47608: F, t47612: F, t47616: F, t47618: F, t47620: F, t5715: F, t5728: F, t9659: F, t1353: F, t198: F, t3829: F, t13607: F, t13656: F, t1450: F, t39419: F, t39422: F, t46297: F, t46963: F, t47753: F, t47754: F, t47758: F, t47759: F, t47760: F, t47798: F, t47828: F, t47862: F, t47889: F, t47922: F, t48153: F, t48155: F, t48157: F, t48159: F, t48160: F, t48218: F, t532: F, t5536: F, t5591: F, t5627: F, t5783: F, t9547: F, t4144: F, t14304: F, t4147: F, t13674: F, t13872: F, t1448: F, t39528: F, t39531: F, t4139: F, t4140: F, t48228: F, t48231: F, t48232: F, t48234: F, t48236: F, t48238: F, t5541: F) -> (F, F) {
        let (t49354, t49361, t49378, t49382, t49386) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2684::<F>(t1897, t40317, t10111, t22, t5759, t49146, t543, t2782, t4100, t48475, t47423, t5741);
        let t49397 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2685::<F>(t3923, t48105, t2782, t47371, t1399, t14122, t4057, t47424, t47427, t47432, t47436, t49205, t49378, t49382, t49386, t5735, t5755, t9891);
        let (t49399, t49403, t49407, t49426) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2686::<F>(t10026, t14141, t14143, t4056, t676, t14066, t1432, t686, t72, t14188, t2439, t2777);
        let t49428 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2687::<F>(t10130, t1399, t14122, t1437, t4004, t47442, t47444, t48438, t49280, t49399, t49403, t49407, t49426, t5659, t5745, t5755, t5767, t820, t9899);
        let (t49429, t49432, t49439, t49446, t49450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2688::<F>(t10073, t14129, t14159, t3964, t9285, t213, t225, t46475, t10019, t14114, t14145, t2482, t4114, t5658);
        let t49456 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2689::<F>(t1399, t14193, t46422, t47450, t47454, t47455, t48760, t49213, t49429, t49432, t49439, t49446, t49450, t5735, t5745, t5755, t9995);
        let t49466 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2690::<F>(t10171, t13921, t14127, t1424, t1427, t1904, t4057, t4118, t46350, t46392, t46398, t46401, t46403, t46412, t46424, t46435, t46496, t46500, t46505, t46510, t46515, t46572, t46583, t46587, t47403, t47407, t47411, t47413, t47417, t47550, t47554, t47558, t47561, t47926, t47929, t47932, t47936, t47938, t47942, t47945, t47948, t47953, t47957, t47961, t47964, t47967, t47992, t48024, t48052, t48058, t48066, t48076, t49174, t49212, t49233, t49238, t49242, t49248, t49252, t49256, t49260, t49263, t49280, t49293, t49310, t49348, t49354, t49361, t49397, t49428, t49456, t5675, t5735, t5745, t5755, t5775, t820, t9912);
        let (t49468, t49472, t49474, t49477, t49480) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2691::<F>(t5600, t9292, t1893, t4075, t786, t9682, t10115, t1894, t14094, t2435, t1358, t2439, t5710, t785);
        let t49506 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2692::<F>(t2782, t4077, t47794, t556, t1426, t5711, t786, t3917, t1424, t14269, t14299, t213, t225, t4071, t4132, t47568, t47570, t47574, t47580, t47591, t49161, t49468, t49472, t49474, t49477, t49480, t561, t5774, t9657);
        let t49534 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2693::<F>(t3899, t5775, t689, t14100, t9686, t13729, t2782, t4131, t556, t47506, t5722, t10171, t1424, t14268, t1444, t4076, t47593, t47595, t47601, t47606, t47608, t47612, t47616, t47618, t47620, t5715, t5728, t9659);
        let t49550 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2694::<F>(t1353, t198, t3829, t13607, t13656, t1450, t39419, t39422, t46297, t46963, t47753, t47754, t47758, t47759, t47760, t47798, t47828, t47862, t47889, t47922, t48153, t48155, t48157, t48159, t48160, t48218, t49466, t49506, t49534, t532, t5536, t5591, t5627, t5783, t9547);
        let t49571 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2695::<F>(t1353, t4144, t14304, t4147, t13674, t13872, t1448, t39528, t39531, t4139, t4140, t48228, t48231, t48232, t48234, t48236, t48238, t5536, t5541);
    (t49550, t49571)
}
