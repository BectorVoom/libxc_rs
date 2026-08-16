//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta840 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3013;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3014;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3015;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3016;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3017;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3018;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3019;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3020;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3021;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3022;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3023;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta840(t1058: f64, t1060: f64, t11051: f64, t14526: f64, t14600: f64, t14618: f64, t1615: f64, t18086: f64, t18099: f64, t18155: f64, t23508: f64, t3040: f64, t3180: f64, t3197: f64, t3200: f64, t3201: f64, t360: f64, t43503: f64, t43515: f64, t43516: f64, t43576: f64, t43577: f64, t4594: f64, t4649: f64, t4674: f64, t4684: f64, t4685: f64, t50465: f64, t50509: f64, t50516: f64, t50592: f64, t5928: f64, t5937: f64, t62925: f64, t1022: f64, t11034: f64, t11046: f64, t11054: f64, t11059: f64, t11065: f64, t11066: f64, t14577: f64, t14587: f64, t14630: f64, t14651: f64, t18047: f64, t18080: f64, t18093: f64, t18107: f64, t18121: f64, t18162: f64, t3120: f64, t3186: f64, t3193: f64, t43480: f64, t4669: f64, t4677: f64, t4681: f64, t5929: f64, t5932: f64, t5936: f64, t1049: f64, t14488: f64, t14578: f64, t14606: f64, t14622: f64, t14640: f64, t14645: f64, t1610: f64, t1625: f64, t1630: f64, t17959: f64, t18103: f64, t18161: f64, t381: f64, t4657: f64, t47841: f64, t50535: f64, t5914: f64, t62757: f64, t1003: f64, t11037: f64, t13940: f64, t14608: f64, t14615: f64, t14648: f64, t1629: f64, t1632: f64, t18088: f64, t18117: f64, t18129: f64, t18150: f64, t3188: f64, t353: f64, t383: f64, t43536: f64, t43558: f64, t4673: f64, t5939: f64, t62914: f64, t62984: f64, t6739: f64, t3185: f64, t61734: f64, t1063: f64, t14572: f64, t14631: f64, t14654: f64, t17671: f64, t17876: f64, t18081: f64, t18108: f64, t3076: f64, t3189: f64, t3204: f64, t384: f64, t4615: f64, t4691: f64, t47853: f64, t50508: f64, t5903: f64, t5941: f64, t62604: f64, t18053: f64, t225: f64, t4693: f64, t10160: f64, t1052: f64, t1055: f64, t1066: f64, t11010: f64, t14529: f64, t14552: f64, t14555: f64, t14659: f64, t17588: f64, t17875: f64, t18166: f64, t3020: f64, t3026: f64, t3174: f64, t3207: f64, t349: f64, t388: f64, t4557: f64, t4665: f64, t4694: f64, t5920: f64, t61646: f64, t62953: f64, t62988: f64, t63022: f64, t990: f64, t1070: f64, t193: f64, t336: f64, t60741: f64, t60744: f64, t60748: f64, t60750: f64, t60752: f64, t60787: f64, t60966: f64, t60970: f64, t61010: f64, t61048: f64, t61643: f64, t62742: f64, t62744: f64, t62748: f64, t62750: f64, t62753: f64, t25: f64, t265: f64, t394: f64, t59618: f64, t60840: f64, t60878: f64, t60904: f64, t60909: f64, t60924: f64, t60939: f64, t60962: f64, t1074: f64, t12606: f64, t13493: f64, t1408: f64, t1409: f64, t14675: f64, t1642: f64, t16557: f64, t16558: f64, t17133: f64, t18176: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t396: f64, t3966: f64, t40: f64, t4705: f64, t47676: f64, t5397: f64, t5398: f64, t55677: f64, t5669: f64, t5955: f64, t59627: f64, t59629: f64, t59631: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t15051: f64, t51667: f64, t4857: f64, t4781: f64, t1118: f64, t3264: f64, t18238: f64, t690: f64, t3247: f64, t1088: f64, t123: f64, t18236: f64, t18231: f64, t2244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t63058 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3013(t1058, t1060, t11051, t14526, t14600, t14618, t1615, t18086, t18099, t18155, t23508, t3040, t3180, t3197, t3200, t3201, t360, t43503, t43515, t43516, t43576, t43577, t4594, t4649, t4674, t4684, t4685, t50465, t50509, t50516, t50592, t5928, t5937, t62925);
        let t63095 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3014(t1022, t1058, t1060, t11034, t11046, t11054, t11059, t11065, t11066, t14577, t14587, t14630, t14651, t18047, t18080, t18086, t18093, t18107, t18121, t18162, t3120, t3180, t3186, t3193, t3200, t43480, t4669, t4677, t4681, t5928, t5929, t5932, t5936);
        let t63133 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3015(t1049, t1058, t1060, t11065, t14488, t14578, t14606, t14622, t14640, t14645, t1610, t1625, t1630, t17959, t18080, t18103, t18161, t3120, t3200, t381, t4649, t4657, t4669, t4684, t47841, t50535, t5914, t5932, t62757);
        let t63168 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3016(t1003, t11037, t11046, t11059, t13940, t14488, t14608, t14615, t14618, t14648, t1629, t1632, t18088, t18117, t18129, t18150, t3120, t3186, t3188, t3200, t353, t360, t383, t43536, t43558, t4673, t4684, t5928, t5939, t62914, t62984, t6739);
        let t63198 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3017(t3185, t61734, t1063, t11037, t14572, t14618, t14622, t14631, t14654, t17671, t17876, t18081, t18108, t18150, t3076, t3189, t3200, t3204, t384, t4615, t4649, t4669, t4684, t4691, t47853, t50508, t50509, t5903, t5936, t5941, t62604);
        let t63235 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3018(t18053, t225, t4693, t10160, t1049, t1052, t1055, t1066, t11010, t14529, t14552, t14555, t14659, t17588, t17875, t18047, t18166, t3020, t3026, t3174, t3207, t349, t388, t4557, t4665, t4694, t5914, t5920, t61646, t62914, t62953, t62988, t63022, t63058, t63095, t63133, t63168, t63198, t990);
        let t63241 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3019(t1070, t193, t336, t60741, t60744, t60748, t60750, t60752, t60787, t60966, t60970, t61010, t61048, t61643, t62742, t62744, t62748, t62750, t62753, t63235);
        let t63261 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3020(t25, t265, t394, t59618, t60840, t60878, t60904, t60909, t60924, t60939, t60962, t63241, t1074, t12606, t13493, t1408, t1409, t14675, t1642, t16557, t16558, t17133, t18176, t2249, t2250, t2756, t3220, t396, t3966, t40, t4705, t47676, t5397, t5398, t55677, t5669, t5955, t59627, t59629, t59631, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let (t63280, t63283, t63287, t63290, t63291) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3021(t15051, t51667, t4857, t4781, t1118, t3264, t18238, t690);
        let (t63294, t63296) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3022(t16558, t3247, t607, t1088, t123);
        let (t63298, t63300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3023(t18236, t2250, t1088, t123);
        let (t63302, t63304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3024(t18231, t2244, t1088, t123);
    (t63261, t63280, t63283, t63287, t63290, t63291, t63294, t63296, t63298, t63300, t63302, t63304)
}
