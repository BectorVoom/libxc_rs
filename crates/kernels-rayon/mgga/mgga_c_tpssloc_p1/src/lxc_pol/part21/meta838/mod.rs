//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta838 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2990;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2991;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2992;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2993;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2994;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2995;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2996;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2997;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2998;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2999;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta838(t17624: f64, t2960: f64, t5884: f64, t698: f64, t973: f64, t5889: f64, t10876: f64, t10937: f64, t10949: f64, t13980: f64, t13985: f64, t13995: f64, t14069: f64, t17637: f64, t17670: f64, t17681: f64, t17714: f64, t3117: f64, t43385: f64, t4582: f64, t50084: f64, t50094: f64, t50098: f64, t50100: f64, t50110: f64, t50113: f64, t50116: f64, t10422: f64, t17676: f64, t3070: f64, t225: f64, t61618: f64, t10413: f64, t14122: f64, t14126: f64, t14489: f64, t1539: f64, t1616: f64, t2979: f64, t3071: f64, t369: f64, t378: f64, t4343: f64, t4650: f64, t49929: f64, t49934: f64, t50132: f64, t50147: f64, t50169: f64, t50172: f64, t50174: f64, t50181: f64, t59715: f64, t59767: f64, t61871: f64, t68: f64, t977: f64, t17171: f64, t2970: f64, t17167: f64, t10390: f64, t14189: f64, t14213: f64, t17923: f64, t18025: f64, t43200: f64, t43214: f64, t43219: f64, t43221: f64, t43361: f64, t4644: f64, t48477: f64, t50183: f64, t50189: f64, t50229: f64, t5873: f64, t59755: f64, t59763: f64, t10231: f64, t17157: f64, t17161: f64, t17183: f64, t17178: f64, t17599: f64, t17602: f64, t17994: f64, t43228: f64, t50242: f64, t50250: f64, t50255: f64, t50258: f64, t50262: f64, t59730: f64, t59746: f64, t18041: f64, t18024: f64, t13969: f64, t17733: f64, t3130: f64, t10214: f64, t1041: f64, t10883: f64, t14080: f64, t14187: f64, t17596: f64, t17697: f64, t17712: f64, t17998: f64, t3039: f64, t3041: f64, t3121: f64, t43248: f64, t43253: f64, t4585: f64, t4588: f64, t48496: f64, t50272: f64, t59751: f64, t61798: f64, t61855: f64, t61910: f64, t59637: f64, t60810: f64, t60812: f64, t60814: f64, t60816: f64, t60821: f64, t60825: f64, t60827: f64, t60829: f64, t60831: f64, t60834: f64, t60836: f64, t60839: f64, t60842: f64, t60844: f64, t60847: f64, t60850: f64, t60852: f64, t60855: f64, t60857: f64, t60860: f64, t60862: f64, t60864: f64, t60866: f64, t60873: f64, t59891: f64, t59958: f64, t59961: f64, t59966: f64, t59968: f64, t59970: f64, t59972: f64, t60886: f64, t60890: f64, t60893: f64, t60899: f64, t60903: f64, t59981: f64, t60006: f64, t60008: f64, t60010: f64, t60016: f64, t60021: f64, t60023: f64, t60025: f64, t60027: f64, t60029: f64, t60033: f64, t60906: f64, t60908: f64, t60035: f64, t60037: f64, t60039: f64, t60041: f64, t60044: f64, t60047: f64, t60050: f64, t60053: f64, t60056: f64, t60354: f64, t60915: f64, t60917: f64, t60359: f64, t60371: f64, t60374: f64, t60377: f64, t60381: f64, t60384: f64, t60387: f64, t60919: f64, t60923: f64, t60930: f64, t60932: f64, t60936: f64, t60938: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t62576 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2990(t17624, t2960, t5884, t698, t973, t5889, t10876, t10937, t10949, t13980, t13985, t13995, t14069, t17637, t17670, t17681, t17714, t3117, t43385, t4582, t50084, t50094, t50098, t50100, t50110, t50113, t50116);
        let (t62604, t62616) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2991(t10422, t17676, t3070, t225, t61618, t10413, t14122, t14126, t14489, t1539, t1616, t2979, t3071, t369, t378, t4343, t4650, t49929, t49934, t50132, t50147, t50169, t50172, t50174, t50181, t59715, t59767, t61871, t68, t973, t977);
        let t62648 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2992(t17171, t2970, t973, t17167, t10390, t10413, t14189, t14213, t17923, t18025, t2979, t3071, t43200, t43214, t43219, t43221, t43361, t4644, t48477, t50183, t50189, t50229, t5873, t59755, t59763, t977);
        let t62680 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2993(t10231, t17157, t973, t17161, t17183, t2970, t17178, t17599, t17602, t17994, t2960, t43228, t50242, t50250, t50255, t50258, t50262, t59730, t59746, t977);
        let t62722 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2994(t10390, t18041, t10422, t18024, t3070, t13969, t17733, t3130, t10214, t1041, t10883, t10937, t14080, t14187, t17596, t17697, t17712, t17998, t2960, t3039, t3041, t3117, t3121, t43248, t43253, t4582, t4585, t4588, t48496, t50272, t59751, t61798, t61855, t61910, t973);
        let t62729 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2995(t59637, t60810, t60812, t60814, t60816, t60821, t60825, t60827, t60829, t60831, t60834, t60836);
        let t62730 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2996(t60839, t60842, t60844, t60847, t60850, t60852, t60855, t60857, t60860, t60862, t60864, t60866, t60873);
        let t62732 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2997(t59891, t59958, t59961, t59966, t59968, t59970, t59972, t60886, t60890, t60893, t60899, t60903);
        let t62733 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2998(t59981, t60006, t60008, t60010, t60016, t60021, t60023, t60025, t60027, t60029, t60033, t60906, t60908);
        let t62736 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2999(t60035, t60037, t60039, t60041, t60044, t60047, t60050, t60053, t60056, t60354, t60915, t60917);
        let t62737 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3000(t60359, t60371, t60374, t60377, t60381, t60384, t60387, t60919, t60923, t60930, t60932, t60936, t60938);
    (t62576, t62604, t62616, t62648, t62680, t62722, t62729, t62730, t62732, t62733, t62736, t62737)
}
