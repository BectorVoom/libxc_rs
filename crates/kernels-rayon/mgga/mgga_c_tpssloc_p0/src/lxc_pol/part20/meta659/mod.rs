//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta659 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2452;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2453;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2454;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2455;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2456;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2457;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2458;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2459;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2460;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2461;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2462;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta659(t1020: f64, t1616: f64, t248: f64, t43216: f64, t10489: f64, t4644: f64, t10898: f64, t4630: f64, t10882: f64, t48569: f64, t10463: f64, t10493: f64, t10517: f64, t10886: f64, t10891: f64, t10937: f64, t10972: f64, t13762: f64, t14080: f64, t14099: f64, t1618: f64, t3098: f64, t42496: f64, t42653: f64, t43186: f64, t4579: f64, t4652: f64, t13961: f64, t3109: f64, t10263: f64, t10321: f64, t10403: f64, t10408: f64, t14122: f64, t3070: f64, t3071: f64, t3132: f64, t42505: f64, t42541: f64, t43200: f64, t43206: f64, t43214: f64, t43219: f64, t43221: f64, t43226: f64, t43241: f64, t4337: f64, t4347: f64, t4609: f64, t13542: f64, t2970: f64, t973: f64, t13546: f64, t10480: f64, t13969: f64, t13986: f64, t3039: f64, t4599: f64, t49850: f64, t10870: f64, t10875: f64, t10879: f64, t10904: f64, t13977: f64, t13987: f64, t14001: f64, t14006: f64, t2960: f64, t42561: f64, t43228: f64, t43233: f64, t47701: f64, t977: f64, t13765: f64, t3040: f64, t607: f64, t883: f64, t1023: f64, t10957: f64, t10962: f64, t14211: f64, t14215: f64, t42388: f64, t42483: f64, t43246: f64, t43248: f64, t43253: f64, t43254: f64, t43361: f64, t4585: f64, t4590: f64, t48611: f64, t48612: f64, t49616: f64, t49976: f64, t10903: f64, t14507: f64, t14651: f64, t3069: f64, t10394: f64, t10952: f64, t13995: f64, t14069: f64, t14077: f64, t14103: f64, t1622: f64, t3073: f64, t3120: f64, t3123: f64, t3134: f64, t42746: f64, t43262: f64, t43273: f64, t43277: f64, t43281: f64, t43285: f64, t10956: f64, t1611: f64, t10459: f64, t4608: f64, t698: f64, t10398: f64, t1041: f64, t10419: f64, t1044: f64, t1046: f64, t14085: f64, t14147: f64, t14187: f64, t14189: f64, t3057: f64, t3117: f64, t43301: f64, t4582: f64, t4588: f64, t45997: f64, t47734: f64, t48554: f64, t2770: f64, t2987: f64, t10277: f64, t4509: f64, t10390: f64, t14501: f64, t10915: f64, t13554: f64, t14033: f64, t14037: f64, t2986: f64, t43303: f64, t43307: f64, t43310: f64, t43313: f64, t4575: f64, t45971: f64, t10305: f64, t10857: f64, t10858: f64, t14041: f64, t1539: f64, t3121: f64, t3130: f64, t3131: f64, t42397: f64, t43325: f64, t43332: f64, t43336: f64, t43341: f64, t43350: f64, t43352: f64, t43354: f64, t4593: f64, t1606: f64, t2402: f64, t10454: f64, t13950: f64, t14202: f64, t3048: f64, t14206: f64, t3108: f64, t1025: f64, t10501: f64, t3064: f64, t43374: f64, t43377: f64, t43382: f64, t43406: f64, t43410: f64, t47775: f64, t48497: f64, t48471: f64, t48511: f64, t48543: f64, t48577: f64, t48622: f64, t48656: f64, t49609: f64, t49654: f64, t49688: f64, t49718: f64, t49750: f64, t49786: f64, t49824: f64, t49860: f64, t49891: f64, t49924: f64, t49961: f64, t49991: f64, t50035: f64, t50066: f64, t50102: f64, t50136: f64, t50176: f64, t3185: f64, t49649: f64, t11031: f64, t11054: f64, t11081: f64, t14578: f64, t14596: f64, t14605: f64, t14608: f64, t14622: f64, t1629: f64, t1630: f64, t3076: f64, t3180: f64, t3186: f64, t3189: f64, t3200: f64, t43473: f64, t43515: f64, t43542: f64, t4669: f64, t4680: f64, t4684: f64, t4691: f64, t47819: f64) -> (f64, f64) {
        let t50207 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2452(t1020, t1616, t248, t43216, t10489, t4644, t10898, t4630, t10882, t48569, t10463, t10493, t10517, t10886, t10891, t10937, t10972, t13762, t14080, t14099, t1618, t3098, t42496, t42653, t43186, t4579, t4652);
        let t50237 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2453(t13961, t3109, t10263, t10321, t10403, t10408, t14122, t1616, t3070, t3071, t3132, t42505, t42541, t43200, t43206, t43214, t43219, t43221, t43226, t43241, t4337, t4347, t4609);
        let (t50242, t50250, t50255, t50259, t50262) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2454(t13542, t2970, t973, t13546, t10480, t13969, t13986, t3039, t4599, t49850, t10870, t4644);
        let t50268 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2455(t50262, t10875, t48569, t10879, t10904, t13977, t13987, t14001, t14006, t2960, t42561, t43228, t43233, t47701, t50242, t50250, t50255, t50259, t973, t977);
        let t50301 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2456(t10937, t13765, t3040, t607, t883, t1023, t10957, t10962, t14211, t14215, t3070, t3071, t42388, t42483, t42505, t43246, t43248, t43253, t43254, t43361, t4337, t4585, t4590, t4652, t48611, t48612, t49616, t49976);
        let t50329 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2457(t10903, t14507, t14651, t3069, t10394, t10403, t10937, t10952, t13995, t14069, t14077, t14103, t14211, t1622, t3071, t3073, t3120, t3123, t3134, t42746, t43262, t43273, t43277, t43281, t43285, t607, t883);
        let t50365 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2458(t10956, t1611, t10517, t4630, t10459, t4644, t4608, t698, t973, t10398, t1041, t10419, t1044, t1046, t13995, t14085, t14147, t14187, t14189, t248, t3057, t3117, t43301, t4582, t4588, t45997, t47734, t48554);
        let t50393 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2459(t2770, t2987, t10277, t4509, t10390, t13765, t10937, t14501, t10408, t10915, t13554, t14033, t14037, t2986, t3070, t42496, t43303, t43307, t43310, t43313, t4575, t45971, t4644, t49976);
        let t50423 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2460(t10305, t10390, t10857, t10858, t10891, t14041, t14103, t1539, t1616, t3070, t3071, t3121, t3130, t3131, t42397, t43325, t43332, t43336, t43341, t43350, t43352, t43354, t4347, t4582, t4593);
        let t50452 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2461(t1606, t2402, t973, t10454, t4644, t13950, t3117, t14202, t3048, t14206, t3108, t1025, t1041, t10501, t14085, t1622, t3064, t3098, t43374, t43377, t43382, t43406, t43410, t4582, t47775, t48497);
        let t50457 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2462(t48471, t48511, t48543, t48577, t48622, t48656, t49609, t49654, t49688, t49718, t49750, t49786, t49824, t49860, t49891, t49924, t49961, t49991, t50035, t50066, t50102, t50136, t50176, t50207, t50237, t50268, t50301, t50329, t50365, t50393, t50423, t50452);
        let t50490 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2463(t3185, t49649, t11031, t11054, t11081, t14578, t14596, t14605, t14608, t14622, t1629, t1630, t3076, t3131, t3180, t3186, t3189, t3200, t43473, t43515, t43542, t4669, t4680, t4684, t4691, t47819);
    (t50457, t50490)
}
