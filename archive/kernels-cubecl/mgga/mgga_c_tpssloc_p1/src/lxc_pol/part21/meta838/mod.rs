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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta838<F: Float>(t17624: F, t2960: F, t5884: F, t698: F, t973: F, t5889: F, t10876: F, t10937: F, t10949: F, t13980: F, t13985: F, t13995: F, t14069: F, t17637: F, t17670: F, t17681: F, t17714: F, t3117: F, t43385: F, t4582: F, t50084: F, t50094: F, t50098: F, t50100: F, t50110: F, t50113: F, t50116: F, t10422: F, t17676: F, t3070: F, t225: F, t61618: F, t10413: F, t14122: F, t14126: F, t14489: F, t1539: F, t1616: F, t2979: F, t3071: F, t369: F, t378: F, t4343: F, t4650: F, t49929: F, t49934: F, t50132: F, t50147: F, t50169: F, t50172: F, t50174: F, t50181: F, t59715: F, t59767: F, t61871: F, t68: F, t977: F, t17171: F, t2970: F, t17167: F, t10390: F, t14189: F, t14213: F, t17923: F, t18025: F, t43200: F, t43214: F, t43219: F, t43221: F, t43361: F, t4644: F, t48477: F, t50183: F, t50189: F, t50229: F, t5873: F, t59755: F, t59763: F, t10231: F, t17157: F, t17161: F, t17183: F, t17178: F, t17599: F, t17602: F, t17994: F, t43228: F, t50242: F, t50250: F, t50255: F, t50258: F, t50262: F, t59730: F, t59746: F, t18041: F, t18024: F, t13969: F, t17733: F, t3130: F, t10214: F, t1041: F, t10883: F, t14080: F, t14187: F, t17596: F, t17697: F, t17712: F, t17998: F, t3039: F, t3041: F, t3121: F, t43248: F, t43253: F, t4585: F, t4588: F, t48496: F, t50272: F, t59751: F, t61798: F, t61855: F, t61910: F, t59637: F, t60810: F, t60812: F, t60814: F, t60816: F, t60821: F, t60825: F, t60827: F, t60829: F, t60831: F, t60834: F, t60836: F, t60839: F, t60842: F, t60844: F, t60847: F, t60850: F, t60852: F, t60855: F, t60857: F, t60860: F, t60862: F, t60864: F, t60866: F, t60873: F, t59891: F, t59958: F, t59961: F, t59966: F, t59968: F, t59970: F, t59972: F, t60886: F, t60890: F, t60893: F, t60899: F, t60903: F, t59981: F, t60006: F, t60008: F, t60010: F, t60016: F, t60021: F, t60023: F, t60025: F, t60027: F, t60029: F, t60033: F, t60906: F, t60908: F, t60035: F, t60037: F, t60039: F, t60041: F, t60044: F, t60047: F, t60050: F, t60053: F, t60056: F, t60354: F, t60915: F, t60917: F, t60359: F, t60371: F, t60374: F, t60377: F, t60381: F, t60384: F, t60387: F, t60919: F, t60923: F, t60930: F, t60932: F, t60936: F, t60938: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t62576 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2990::<F>(t17624, t2960, t5884, t698, t973, t5889, t10876, t10937, t10949, t13980, t13985, t13995, t14069, t17637, t17670, t17681, t17714, t3117, t43385, t4582, t50084, t50094, t50098, t50100, t50110, t50113, t50116);
        let (t62604, t62616) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2991::<F>(t10422, t17676, t3070, t225, t61618, t10413, t14122, t14126, t14489, t1539, t1616, t2979, t3071, t369, t378, t4343, t4650, t49929, t49934, t50132, t50147, t50169, t50172, t50174, t50181, t59715, t59767, t61871, t68, t973, t977);
        let t62648 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2992::<F>(t17171, t2970, t973, t17167, t10390, t10413, t14189, t14213, t17923, t18025, t2979, t3071, t43200, t43214, t43219, t43221, t43361, t4644, t48477, t50183, t50189, t50229, t5873, t59755, t59763, t977);
        let t62680 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2993::<F>(t10231, t17157, t973, t17161, t17183, t2970, t17178, t17599, t17602, t17994, t2960, t43228, t50242, t50250, t50255, t50258, t50262, t59730, t59746, t977);
        let t62722 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2994::<F>(t10390, t18041, t10422, t18024, t3070, t13969, t17733, t3130, t10214, t1041, t10883, t10937, t14080, t14187, t17596, t17697, t17712, t17998, t2960, t3039, t3041, t3117, t3121, t43248, t43253, t4582, t4585, t4588, t48496, t50272, t59751, t61798, t61855, t61910, t973);
        let t62729 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2995::<F>(t59637, t60810, t60812, t60814, t60816, t60821, t60825, t60827, t60829, t60831, t60834, t60836);
        let t62730 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2996::<F>(t60839, t60842, t60844, t60847, t60850, t60852, t60855, t60857, t60860, t60862, t60864, t60866, t60873);
        let t62732 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2997::<F>(t59891, t59958, t59961, t59966, t59968, t59970, t59972, t60886, t60890, t60893, t60899, t60903);
        let t62733 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2998::<F>(t59981, t60006, t60008, t60010, t60016, t60021, t60023, t60025, t60027, t60029, t60033, t60906, t60908);
        let t62736 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2999::<F>(t60035, t60037, t60039, t60041, t60044, t60047, t60050, t60053, t60056, t60354, t60915, t60917);
        let t62737 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3000::<F>(t60359, t60371, t60374, t60377, t60381, t60384, t60387, t60919, t60923, t60930, t60932, t60936, t60938);
    (t62576, t62604, t62616, t62648, t62680, t62722, t62729, t62730, t62732, t62733, t62736, t62737)
}
