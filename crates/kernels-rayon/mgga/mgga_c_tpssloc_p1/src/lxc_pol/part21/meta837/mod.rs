//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta837 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2978;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2979;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2980;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2981;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2982;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2983;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2984;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2985;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2986;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2987;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2988;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta837(t13965: f64, t4641: f64, t17659: f64, t3048: f64, t14207: f64, t4630: f64, t13969: f64, t17717: f64, t3039: f64, t1020: f64, t10508: f64, t248: f64, t5867: f64, t5878: f64, t1041: f64, t10863: f64, t13980: f64, t14085: f64, t14107: f64, t14180: f64, t17693: f64, t17712: f64, t3117: f64, t3130: f64, t4582: f64, t4585: f64, t4644: f64, t49734: f64, t49748: f64, t49854: f64, t50193: f64, t5861: f64, t61855: f64, t17696: f64, t1021: f64, t10390: f64, t10413: f64, t14211: f64, t17681: f64, t17688: f64, t17925: f64, t17976: f64, t17991: f64, t2780: f64, t2960: f64, t2986: f64, t3071: f64, t360: f64, t42546: f64, t42610: f64, t42613: f64, t43361: f64, t48477: f64, t48611: f64, t49757: f64, t50366: f64, t55677: f64, t55716: f64, t59659: f64, t61719: f64, t973: f64, t974: f64, t977: f64, t998: f64, t10422: f64, t17648: f64, t3070: f64, t10214: f64, t1031: f64, t17701: f64, t17877: f64, t18036: f64, t2979: f64, t378: f64, t42508: f64, t42541: f64, t49799: f64, t49801: f64, t49808: f64, t49810: f64, t49818: f64, t49820: f64, t59668: f64, t59672: f64, t59696: f64, t59725: f64, t59742: f64, t14080: f64, t4571: f64, t14202: f64, t1043: f64, t1615: f64, t375: f64, t10408: f64, t1044: f64, t10957: f64, t10965: f64, t14229: f64, t17890: f64, t2771: f64, t42721: f64, t49822: f64, t49827: f64, t49829: f64, t49831: f64, t49846: f64, t5857: f64, t59682: f64, t59690: f64, t62064: f64, t17700: f64, t1023: f64, t10403: f64, t13611: f64, t1616: f64, t42397: f64, t42735: f64, t42752: f64, t4600: f64, t48607: f64, t49743: f64, t49852: f64, t49871: f64, t49873: f64, t49877: f64, t49884: f64, t49887: f64, t5873: f64, t61524: f64, t62091: f64, t1036: f64, t17878: f64, t17631: f64, t3082: f64, t5905: f64, t10937: f64, t10952: f64, t17632: f64, t17677: f64, t17960: f64, t43110: f64, t48585: f64, t49889: f64, t49892: f64, t49894: f64, t49897: f64, t49906: f64, t49922: f64, t50370: f64, t884: f64, t10480: f64, t10883: f64, t13985: f64, t17670: f64, t17705: f64, t17980: f64, t2776: f64, t3041: f64, t3121: f64, t3132: f64, t42347: f64, t42354: f64, t42358: f64, t42496: f64, t49940: f64, t49945: f64, t49957: f64, t49959: f64, t49964: f64, t49966: f64, t5909: f64, t18035: f64, t10904: f64, t13995: f64, t14033: f64, t14037: f64, t14174: f64, t17734: f64, t17988: f64, t18021: f64, t42505: f64, t43114: f64, t4590: f64, t49972: f64, t49987: f64, t49989: f64, t49993: f64, t5681: f64, t17906: f64, t43338: f64, t5677: f64, t1022: f64, t13532: f64, t13537: f64, t13542: f64, t17593: f64, t17923: f64, t18016: f64, t18025: f64, t18030: f64, t2775: f64, t3123: f64, t3131: f64, t4347: f64, t49616: f64, t50027: f64, t5900: f64, t62055: f64, t62059: f64, t43198: f64, t5908: f64, t18041: f64, t17636: f64, t13528: f64, t14143: f64, t14147: f64, t14184: f64, t14489: f64, t1622: f64, t17718: f64, t17738: f64, t43358: f64, t4593: f64, t48432: f64, t50047: f64, t50056: f64, t17642: f64, t1618: f64, t17920: f64, t42511: f64, t43155: f64, t43157: f64, t43161: f64, t4596: f64, t50062: f64, t50077: f64, t50302: f64, t50445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62148, t62150, t62152, t62164, t62177) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2978(t13965, t4641, t17659, t3048, t14207, t4630, t13969, t17717, t3039, t1020, t10508, t248, t5867);
        let t62185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2979(t10508, t248, t3039, t5878, t1041, t10863, t13980, t14085, t14107, t14180, t17693, t17712, t3117, t3130, t4582, t4585, t4644, t49734, t49748, t49854, t50193, t5861, t61855, t62148, t62150, t62152, t62164, t62177);
        let t62225 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2980(t1041, t13969, t17696, t1021, t10390, t10413, t14211, t17681, t17688, t17925, t17976, t17991, t248, t2780, t2960, t2986, t3039, t3071, t3117, t360, t42546, t42610, t42613, t43361, t48477, t48611, t49757, t50366, t55677, t55716, t5878, t59659, t61719, t973, t974, t977, t998);
        let t62258 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2981(t10422, t17648, t3070, t10214, t1031, t17701, t17877, t18036, t2979, t378, t42508, t42541, t49799, t49801, t49808, t49810, t49818, t49820, t59668, t59672, t59696, t59725, t59742, t973, t977);
        let (t62291, t62296) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2982(t14080, t4571, t14202, t4644, t1043, t1615, t375, t10408, t1041, t1044, t10957, t10965, t14229, t17890, t248, t2771, t2780, t3070, t3071, t3117, t42721, t49822, t49827, t49829, t49831, t49846, t5857, t5861, t5867, t59682, t59690, t62064);
        let t62333 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2983(t10413, t10422, t17700, t1023, t10403, t10408, t13611, t1616, t2771, t2780, t3039, t3070, t3071, t42397, t42735, t42752, t4582, t4600, t48607, t49743, t49852, t49871, t49873, t49877, t49884, t49887, t5873, t61524, t62091);
        let t62362 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2984(t1036, t17878, t13969, t17631, t3039, t3082, t5905, t10937, t10952, t17632, t17677, t17960, t2986, t3070, t3071, t43110, t48585, t49889, t49892, t49894, t49897, t49906, t49922, t50370, t55716, t884);
        let t62398 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2985(t10390, t10403, t10480, t10883, t13985, t17670, t17677, t17705, t17712, t17980, t2776, t3041, t3071, t3121, t3132, t42347, t42354, t42358, t42496, t4582, t49940, t49945, t49957, t49959, t49964, t49966, t5873, t5909);
        let t62427 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2986(t10403, t10422, t18035, t10904, t10937, t13995, t14033, t14037, t14085, t14174, t17734, t17988, t18021, t18036, t2960, t3070, t3071, t3121, t42505, t43114, t4590, t4644, t49972, t49987, t49989, t49993, t5681);
        let t62475 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2987(t17906, t3048, t1041, t248, t43338, t5677, t1022, t10403, t10408, t10413, t10937, t10957, t13532, t13537, t13542, t14211, t1616, t17593, t17923, t18016, t18025, t18030, t2775, t2960, t3070, t3071, t3123, t3131, t42397, t42505, t42541, t4347, t49616, t50027, t5900, t62055, t62059, t62291);
        let t62512 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2988(t3070, t43198, t5908, t10937, t18041, t1041, t13969, t17636, t10408, t10413, t10952, t13528, t14143, t14147, t14184, t14489, t1616, t1622, t17718, t17738, t2776, t2960, t3039, t3071, t43358, t4582, t4593, t4644, t48432, t50047, t50056, t5878, t5909);
        let t62544 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2989(t1041, t13969, t17642, t17906, t3117, t10390, t10403, t10413, t10965, t1618, t17920, t17976, t3041, t3048, t3071, t3132, t42511, t43155, t43157, t43161, t4596, t50062, t50077, t50302, t50445, t5681, t5900, t5909);
    (t62185, t62225, t62258, t62296, t62333, t62362, t62398, t62427, t62475, t62512, t62544)
}
