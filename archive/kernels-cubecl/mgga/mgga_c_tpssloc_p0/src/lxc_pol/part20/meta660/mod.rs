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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta660<F: Float>(t10470: F, t11058: F, t381: F, t1615: F, t6739: F, t10482: F, t3120: F, t11064: F, t1057: F, t49864: F, t3040: F, t4657: F, t1022: F, t1058: F, t1060: F, t1061: F, t11037: F, t11046: F, t11051: F, t11078: F, t14526: F, t14595: F, t14627: F, t14630: F, t14645: F, t14651: F, t3180: F, t3186: F, t3188: F, t3197: F, t4669: F, t4673: F, t4677: F, t4678: F, t4680: F, t3199: F, t49649: F, t11045: F, t1003: F, t10359: F, t11043: F, t14574: F, t14586: F, t14640: F, t1610: F, t1632: F, t3200: F, t3201: F, t3202: F, t3204: F, t4615: F, t4684: F, t4689: F, t49599: F, t14538: F, t225: F, t14536: F, t10164: F, t1634: F, t14532: F, t10167: F, t10181: F, t10358: F, t10481: F, t1049: F, t1052: F, t1055: F, t1063: F, t1066: F, t10857: F, t11007: F, t11024: F, t11034: F, t11054: F, t11055: F, t11059: F, t11060: F, t11065: F, t11066: F, t13940: F, t14488: F, t14529: F, t14552: F, t14571: F, t14572: F, t14577: F, t14581: F, t14587: F, t14590: F, t14591: F, t14600: F, t14605: F, t14606: F, t14615: F, t14618: F, t14623: F, t14631: F, t14648: F, t14659: F, t1603: F, t1625: F, t1629: F, t1932: F, t25757: F, t3166: F, t3169: F, t3176: F, t3193: F, t3207: F, t353: F, t360: F, t383: F, t384: F, t388: F, t43470: F, t43503: F, t43553: F, t43554: F, t43562: F, t43576: F, t43577: F, t4552: F, t4557: F, t4649: F, t4681: F, t47819: F, t47844: F, t47867: F, t48428: F, t49588: F, t50014: F, t50457: F, t50490: F, t14562: F, t10160: F, t10170: F, t10182: F, t11010: F, t11084: F, t11085: F, t14545: F, t14549: F, t1635: F, t3020: F, t3026: F, t3174: F, t43431: F, t4660: F, t4665: F, t4694: F, t14527: F, t14534: F, t10165: F, t10166: F, t13743: F, t14555: F, t3175: F, t43599: F, t43604: F, t4693: F, t48427: F, t1065: F, t13736: F, t13939: F, t14658: F, t3206: F, t349: F, t43440: F, t43619: F, t990: F, t1070: F, t193: F, t336: F, t47793: F, t47795: F, t47798: F, t47802: F, t48679: F, t48681: F, t48725: F, t48727: F, t48730: F, t48732: F, t11094: F, t3213: F, t4696: F, t4700: F, t48734: F, t48736: F, t48738: F, t48741: F, t48744: F, t48747: F, t48750: F, t48753: F, t48755: F, t48759: F, t48762: F, t48765: F, t48768: F, t48770: F, t49496: F, t49499: F, t49502: F, t49506: F, t49508: F, t49510: F, t49512: F, t1068: F, t11087: F, t14662: F, t3216: F, t4701: F, t49068: F, t49071: F, t49075: F, t49080: F, t49517: F, t49520: F, t49522: F, t49525: F, t49529: F, t11091: F, t1637: F, t43637: F, t49082: F, t49084: F, t49086: F, t49088: F, t49090: F, t49092: F, t49095: F, t49535: F, t49538: F, t49540: F, t3209: F, t13666: F, t14667: F, t49228: F, t49544: F, t49548: F, t49550: F, t49552: F, t49556: F, t49558: F, t49560: F, t49562: F) -> (F, F, F, F, F, F) {
        let (t50508, t50509, t50510, t50516, t50535, t50540) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2464::<F>(t10470, t11058, t381, t1615, t6739, t10482, t3120, t11064, t1057, t49864, t3040, t4657);
        let t50560 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2465::<F>(t1022, t1058, t1060, t1061, t11037, t11046, t11051, t11078, t14526, t14595, t14627, t14630, t14645, t14651, t3180, t3186, t3188, t3197, t4669, t4673, t4677, t4678, t4680, t50535, t50540);
        let t50616 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2466::<F>(t3199, t49649, t10470, t11045, t381, t1003, t10359, t11037, t11043, t11051, t14574, t14586, t14595, t14640, t1610, t1632, t3200, t3201, t3202, t3204, t4615, t4684, t4689, t49599, t50509, t50540);
        let t50648 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2467::<F>(t14538, t225, t14536, t10164, t1634, t14532, t10167, t10181, t1022, t10358, t10481, t1049, t1052, t1055, t1058, t1060, t1063, t1066, t10857, t11007, t11024, t11034, t11037, t11051, t11054, t11055, t11059, t11060, t11065, t11066, t13940, t14488, t14529, t14552, t14571, t14572, t14577, t14581, t14586, t14587, t14590, t14591, t14600, t14605, t14606, t14615, t14618, t14623, t14631, t14648, t14651, t14659, t1603, t1615, t1625, t1629, t1932, t25757, t3120, t3166, t3169, t3176, t3180, t3186, t3188, t3193, t3200, t3207, t353, t360, t381, t383, t384, t388, t43470, t43503, t43553, t43554, t43562, t43576, t43577, t4552, t4557, t4649, t4657, t4673, t4677, t4680, t4681, t4684, t47819, t47844, t47867, t48428, t49588, t50014, t50457, t50490, t50508, t50509, t50510, t50516, t50560, t50616);
        let t50678 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2468::<F>(t14562, t225, t10160, t10170, t10182, t1052, t1066, t11010, t11084, t11085, t14529, t14545, t14549, t1634, t1635, t3020, t3026, t3174, t3207, t388, t43431, t4657, t4660, t4665, t4694);
        let t50712 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2469::<F>(t14527, t225, t14534, t10165, t10166, t10167, t10170, t1052, t1066, t13743, t14549, t14555, t14659, t1634, t1635, t3026, t3169, t3175, t3207, t381, t388, t43599, t43604, t4660, t4665, t4693, t48427);
        let t50744 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2470::<F>(t10160, t10182, t1049, t1052, t1065, t11085, t13736, t13939, t14526, t14545, t14555, t14658, t1635, t3026, t3169, t3174, t3176, t3206, t349, t388, t43440, t43619, t4557, t4693, t4694, t50457, t990);
        let t50750 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2471::<F>(t1070, t193, t336, t47793, t47795, t47798, t47802, t48679, t48681, t48725, t48727, t48730, t48732, t50648, t50678, t50712, t50744);
        let t50755 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2472::<F>(t11094, t3213, t4696, t4700, t48734, t48736, t48738, t48741, t48744, t48747, t48750, t48753, t48755, t48759);
        let (t50757, t50764) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2473::<F>(t48762, t48765, t48768, t48770, t49496, t49499, t49502, t49506, t49508, t49510, t49512, t1068, t11087, t14662, t3216, t4700, t4701, t49068, t49071, t49075, t49080, t49517, t49520, t49522, t49525, t49529);
        let t50771 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2474::<F>(t11091, t1637, t43637, t4700, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49535, t49538, t49540);
        let t50779 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2475::<F>(t1068, t3209, t13666, t14667, t4700, t49228, t49544, t49548, t49550, t49552, t49556, t49558, t49560, t49562);
    (t50750, t50755, t50757, t50764, t50771, t50779)
}
