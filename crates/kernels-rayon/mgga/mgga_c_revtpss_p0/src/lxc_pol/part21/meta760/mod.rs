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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta760(t1897: f64, t40317: f64, t10111: f64, t22: f64, t5759: f64, t49146: f64, t543: f64, t2782: f64, t4100: f64, t48475: f64, t47423: f64, t5741: f64, t3923: f64, t48105: f64, t47371: f64, t1399: f64, t14122: f64, t4057: f64, t47424: f64, t47427: f64, t47432: f64, t47436: f64, t49205: f64, t5735: f64, t5755: f64, t9891: f64, t10026: f64, t14141: f64, t14143: f64, t4056: f64, t676: f64, t14066: f64, t1432: f64, t686: f64, t72: f64, t14188: f64, t2439: f64, t2777: f64, t10130: f64, t1437: f64, t4004: f64, t47442: f64, t47444: f64, t48438: f64, t49280: f64, t5659: f64, t5745: f64, t5767: f64, t820: f64, t9899: f64, t10073: f64, t14129: f64, t14159: f64, t3964: f64, t9285: f64, t213: f64, t225: f64, t46475: f64, t10019: f64, t14114: f64, t14145: f64, t2482: f64, t4114: f64, t5658: f64, t14193: f64, t46422: f64, t47450: f64, t47454: f64, t47455: f64, t48760: f64, t49213: f64, t9995: f64, t10171: f64, t13921: f64, t14127: f64, t1424: f64, t1427: f64, t1904: f64, t4118: f64, t46350: f64, t46392: f64, t46398: f64, t46401: f64, t46403: f64, t46412: f64, t46424: f64, t46435: f64, t46496: f64, t46500: f64, t46505: f64, t46510: f64, t46515: f64, t46572: f64, t46583: f64, t46587: f64, t47403: f64, t47407: f64, t47411: f64, t47413: f64, t47417: f64, t47550: f64, t47554: f64, t47558: f64, t47561: f64, t47926: f64, t47929: f64, t47932: f64, t47936: f64, t47938: f64, t47942: f64, t47945: f64, t47948: f64, t47953: f64, t47957: f64, t47961: f64, t47964: f64, t47967: f64, t47992: f64, t48024: f64, t48052: f64, t48058: f64, t48066: f64, t48076: f64, t49174: f64, t49212: f64, t49233: f64, t49238: f64, t49242: f64, t49248: f64, t49252: f64, t49256: f64, t49260: f64, t49263: f64, t49293: f64, t49310: f64, t49348: f64, t5675: f64, t5775: f64, t9912: f64, t5600: f64, t9292: f64, t1893: f64, t4075: f64, t786: f64, t9682: f64, t10115: f64, t1894: f64, t14094: f64, t2435: f64, t1358: f64, t5710: f64, t785: f64, t4077: f64, t47794: f64, t556: f64, t1426: f64, t5711: f64, t3917: f64, t14269: f64, t14299: f64, t4071: f64, t4132: f64, t47568: f64, t47570: f64, t47574: f64, t47580: f64, t47591: f64, t49161: f64, t561: f64, t5774: f64, t9657: f64, t3899: f64, t689: f64, t14100: f64, t9686: f64, t13729: f64, t4131: f64, t47506: f64, t5722: f64, t14268: f64, t1444: f64, t4076: f64, t47593: f64, t47595: f64, t47601: f64, t47606: f64, t47608: f64, t47612: f64, t47616: f64, t47618: f64, t47620: f64, t5715: f64, t5728: f64, t9659: f64, t1353: f64, t198: f64, t3829: f64, t13607: f64, t13656: f64, t1450: f64, t39419: f64, t39422: f64, t46297: f64, t46963: f64, t47753: f64, t47754: f64, t47758: f64, t47759: f64, t47760: f64, t47798: f64, t47828: f64, t47862: f64, t47889: f64, t47922: f64, t48153: f64, t48155: f64, t48157: f64, t48159: f64, t48160: f64, t48218: f64, t532: f64, t5536: f64, t5591: f64, t5627: f64, t5783: f64, t9547: f64, t4144: f64, t14304: f64, t4147: f64, t13674: f64, t13872: f64, t1448: f64, t39528: f64, t39531: f64, t4139: f64, t4140: f64, t48228: f64, t48231: f64, t48232: f64, t48234: f64, t48236: f64, t48238: f64, t5541: f64) -> (f64, f64) {
        let (t49354, t49361, t49378, t49382, t49386) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2684(t1897, t40317, t10111, t22, t5759, t49146, t543, t2782, t4100, t48475, t47423, t5741);
        let t49397 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2685(t3923, t48105, t2782, t47371, t1399, t14122, t4057, t47424, t47427, t47432, t47436, t49205, t49378, t49382, t49386, t5735, t5755, t9891);
        let (t49399, t49403, t49407, t49426) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2686(t10026, t14141, t14143, t4056, t676, t14066, t1432, t686, t72, t14188, t2439, t2777);
        let t49428 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2687(t10130, t1399, t14122, t1437, t4004, t47442, t47444, t48438, t49280, t49399, t49403, t49407, t49426, t5659, t5745, t5755, t5767, t820, t9899);
        let (t49429, t49432, t49439, t49446, t49450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2688(t10073, t14129, t14159, t3964, t9285, t213, t225, t46475, t10019, t14114, t14145, t2482, t4114, t5658);
        let t49456 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2689(t1399, t14193, t46422, t47450, t47454, t47455, t48760, t49213, t49429, t49432, t49439, t49446, t49450, t5735, t5745, t5755, t9995);
        let t49466 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2690(t10171, t13921, t14127, t1424, t1427, t1904, t4057, t4118, t46350, t46392, t46398, t46401, t46403, t46412, t46424, t46435, t46496, t46500, t46505, t46510, t46515, t46572, t46583, t46587, t47403, t47407, t47411, t47413, t47417, t47550, t47554, t47558, t47561, t47926, t47929, t47932, t47936, t47938, t47942, t47945, t47948, t47953, t47957, t47961, t47964, t47967, t47992, t48024, t48052, t48058, t48066, t48076, t49174, t49212, t49233, t49238, t49242, t49248, t49252, t49256, t49260, t49263, t49280, t49293, t49310, t49348, t49354, t49361, t49397, t49428, t49456, t5675, t5735, t5745, t5755, t5775, t820, t9912);
        let (t49468, t49472, t49474, t49477, t49480) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2691(t5600, t9292, t1893, t4075, t786, t9682, t10115, t1894, t14094, t2435, t1358, t2439, t5710, t785);
        let t49506 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2692(t2782, t4077, t47794, t556, t1426, t5711, t786, t3917, t1424, t14269, t14299, t213, t225, t4071, t4132, t47568, t47570, t47574, t47580, t47591, t49161, t49468, t49472, t49474, t49477, t49480, t561, t5774, t9657);
        let t49534 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2693(t3899, t5775, t689, t14100, t9686, t13729, t2782, t4131, t556, t47506, t5722, t10171, t1424, t14268, t1444, t4076, t47593, t47595, t47601, t47606, t47608, t47612, t47616, t47618, t47620, t5715, t5728, t9659);
        let t49550 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2694(t1353, t198, t3829, t13607, t13656, t1450, t39419, t39422, t46297, t46963, t47753, t47754, t47758, t47759, t47760, t47798, t47828, t47862, t47889, t47922, t48153, t48155, t48157, t48159, t48160, t48218, t49466, t49506, t49534, t532, t5536, t5591, t5627, t5783, t9547);
        let t49571 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2695(t1353, t4144, t14304, t4147, t13674, t13872, t1448, t39528, t39531, t4139, t4140, t48228, t48231, t48232, t48234, t48236, t48238, t5536, t5541);
    (t49550, t49571)
}
