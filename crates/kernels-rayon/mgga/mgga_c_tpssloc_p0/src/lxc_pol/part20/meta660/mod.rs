//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta660 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2464;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2465;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2466;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2467;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2468;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2469;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2470;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2471;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2472;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2473;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2474;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta660(t10470: f64, t11058: f64, t381: f64, t1615: f64, t6739: f64, t10482: f64, t3120: f64, t11064: f64, t1057: f64, t49864: f64, t3040: f64, t4657: f64, t1022: f64, t1058: f64, t1060: f64, t1061: f64, t11037: f64, t11046: f64, t11051: f64, t11078: f64, t14526: f64, t14595: f64, t14627: f64, t14630: f64, t14645: f64, t14651: f64, t3180: f64, t3186: f64, t3188: f64, t3197: f64, t4669: f64, t4673: f64, t4677: f64, t4678: f64, t4680: f64, t3199: f64, t49649: f64, t11045: f64, t1003: f64, t10359: f64, t11043: f64, t14574: f64, t14586: f64, t14640: f64, t1610: f64, t1632: f64, t3200: f64, t3201: f64, t3202: f64, t3204: f64, t4615: f64, t4684: f64, t4689: f64, t49599: f64, t14538: f64, t225: f64, t14536: f64, t10164: f64, t1634: f64, t14532: f64, t10167: f64, t10181: f64, t10358: f64, t10481: f64, t1049: f64, t1052: f64, t1055: f64, t1063: f64, t1066: f64, t10857: f64, t11007: f64, t11024: f64, t11034: f64, t11054: f64, t11055: f64, t11059: f64, t11060: f64, t11065: f64, t11066: f64, t13940: f64, t14488: f64, t14529: f64, t14552: f64, t14571: f64, t14572: f64, t14577: f64, t14581: f64, t14587: f64, t14590: f64, t14591: f64, t14600: f64, t14605: f64, t14606: f64, t14615: f64, t14618: f64, t14623: f64, t14631: f64, t14648: f64, t14659: f64, t1603: f64, t1625: f64, t1629: f64, t1932: f64, t25757: f64, t3166: f64, t3169: f64, t3176: f64, t3193: f64, t3207: f64, t353: f64, t360: f64, t383: f64, t384: f64, t388: f64, t43470: f64, t43503: f64, t43553: f64, t43554: f64, t43562: f64, t43576: f64, t43577: f64, t4552: f64, t4557: f64, t4649: f64, t4681: f64, t47819: f64, t47844: f64, t47867: f64, t48428: f64, t49588: f64, t50014: f64, t50457: f64, t50490: f64, t14562: f64, t10160: f64, t10170: f64, t10182: f64, t11010: f64, t11084: f64, t11085: f64, t14545: f64, t14549: f64, t1635: f64, t3020: f64, t3026: f64, t3174: f64, t43431: f64, t4660: f64, t4665: f64, t4694: f64, t14527: f64, t14534: f64, t10165: f64, t10166: f64, t13743: f64, t14555: f64, t3175: f64, t43599: f64, t43604: f64, t4693: f64, t48427: f64, t1065: f64, t13736: f64, t13939: f64, t14658: f64, t3206: f64, t349: f64, t43440: f64, t43619: f64, t990: f64, t1070: f64, t193: f64, t336: f64, t47793: f64, t47795: f64, t47798: f64, t47802: f64, t48679: f64, t48681: f64, t48725: f64, t48727: f64, t48730: f64, t48732: f64, t11094: f64, t3213: f64, t4696: f64, t4700: f64, t48734: f64, t48736: f64, t48738: f64, t48741: f64, t48744: f64, t48747: f64, t48750: f64, t48753: f64, t48755: f64, t48759: f64, t48762: f64, t48765: f64, t48768: f64, t48770: f64, t49496: f64, t49499: f64, t49502: f64, t49506: f64, t49508: f64, t49510: f64, t49512: f64, t1068: f64, t11087: f64, t14662: f64, t3216: f64, t4701: f64, t49068: f64, t49071: f64, t49075: f64, t49080: f64, t49517: f64, t49520: f64, t49522: f64, t49525: f64, t49529: f64, t11091: f64, t1637: f64, t43637: f64, t49082: f64, t49084: f64, t49086: f64, t49088: f64, t49090: f64, t49092: f64, t49095: f64, t49535: f64, t49538: f64, t49540: f64, t3209: f64, t13666: f64, t14667: f64, t49228: f64, t49544: f64, t49548: f64, t49550: f64, t49552: f64, t49556: f64, t49558: f64, t49560: f64, t49562: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t50508, t50509, t50510, t50516, t50535, t50540) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2464(t10470, t11058, t381, t1615, t6739, t10482, t3120, t11064, t1057, t49864, t3040, t4657);
        let t50560 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2465(t1022, t1058, t1060, t1061, t11037, t11046, t11051, t11078, t14526, t14595, t14627, t14630, t14645, t14651, t3180, t3186, t3188, t3197, t4669, t4673, t4677, t4678, t4680, t50535, t50540);
        let t50616 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2466(t3199, t49649, t10470, t11045, t381, t1003, t10359, t11037, t11043, t11051, t14574, t14586, t14595, t14640, t1610, t1632, t3200, t3201, t3202, t3204, t4615, t4684, t4689, t49599, t50509, t50540);
        let t50648 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2467(t14538, t225, t14536, t10164, t1634, t14532, t10167, t10181, t1022, t10358, t10481, t1049, t1052, t1055, t1058, t1060, t1063, t1066, t10857, t11007, t11024, t11034, t11037, t11051, t11054, t11055, t11059, t11060, t11065, t11066, t13940, t14488, t14529, t14552, t14571, t14572, t14577, t14581, t14586, t14587, t14590, t14591, t14600, t14605, t14606, t14615, t14618, t14623, t14631, t14648, t14651, t14659, t1603, t1615, t1625, t1629, t1932, t25757, t3120, t3166, t3169, t3176, t3180, t3186, t3188, t3193, t3200, t3207, t353, t360, t381, t383, t384, t388, t43470, t43503, t43553, t43554, t43562, t43576, t43577, t4552, t4557, t4649, t4657, t4673, t4677, t4680, t4681, t4684, t47819, t47844, t47867, t48428, t49588, t50014, t50457, t50490, t50508, t50509, t50510, t50516, t50560, t50616);
        let t50678 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2468(t14562, t225, t10160, t10170, t10182, t1052, t1066, t11010, t11084, t11085, t14529, t14545, t14549, t1634, t1635, t3020, t3026, t3174, t3207, t388, t43431, t4657, t4660, t4665, t4694);
        let t50712 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2469(t14527, t225, t14534, t10165, t10166, t10167, t10170, t1052, t1066, t13743, t14549, t14555, t14659, t1634, t1635, t3026, t3169, t3175, t3207, t381, t388, t43599, t43604, t4660, t4665, t4693, t48427);
        let t50744 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2470(t10160, t10182, t1049, t1052, t1065, t11085, t13736, t13939, t14526, t14545, t14555, t14658, t1635, t3026, t3169, t3174, t3176, t3206, t349, t388, t43440, t43619, t4557, t4693, t4694, t50457, t990);
        let t50750 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2471(t1070, t193, t336, t47793, t47795, t47798, t47802, t48679, t48681, t48725, t48727, t48730, t48732, t50648, t50678, t50712, t50744);
        let t50755 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2472(t11094, t3213, t4696, t4700, t48734, t48736, t48738, t48741, t48744, t48747, t48750, t48753, t48755, t48759);
        let (t50757, t50764) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2473(t48762, t48765, t48768, t48770, t49496, t49499, t49502, t49506, t49508, t49510, t49512, t1068, t11087, t14662, t3216, t4700, t4701, t49068, t49071, t49075, t49080, t49517, t49520, t49522, t49525, t49529);
        let t50771 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2474(t11091, t1637, t43637, t4700, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49535, t49538, t49540);
        let t50779 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2475(t1068, t3209, t13666, t14667, t4700, t49228, t49544, t49548, t49550, t49552, t49556, t49558, t49560, t49562);
    (t50750, t50755, t50757, t50764, t50771, t50779)
}
