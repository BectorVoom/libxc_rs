//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta847 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3063;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3064;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3065;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3066;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3067;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3068;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta847(t18834: f64, t3315: f64, t1117: f64, t3313: f64, t18258: f64, t3307: f64, t1147: f64, t18710: f64, t3400: f64, t6063: f64, t1157: f64, t15121: f64, t15133: f64, t1695: f64, t18899: f64, t3396: f64, t3404: f64, t44300: f64, t4835: f64, t4858: f64, t51366: f64, t6056: f64, t63563: f64, t63566: f64, t63568: f64, t63571: f64, t63574: f64, t63576: f64, t63579: f64, t63582: f64, t63585: f64, t63587: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63323: f64, t43748: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t63327: f64, t63330: f64, t63332: f64, t63334: f64, t63336: f64, t43780: f64, t43782: f64, t43816: f64, t43942: f64, t50952: f64, t50954: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64, t449: f64, t11275: f64, t11277: f64, t3265: f64, t6020: f64, t11297: f64, t11350: f64, t11352: f64, t11356: f64, t11361: f64, t11415: f64, t1155: f64, t15117: f64, t15153: f64, t15156: f64, t15207: f64, t18606: f64, t18609: f64, t18612: f64, t18616: f64, t18647: f64, t18650: f64, t18651: f64, t18786: f64, t3333: f64, t3351: f64, t3357: f64, t3376: f64, t44172: f64, t44177: f64, t44179: f64, t44202: f64, t44205: f64, t4802: f64, t4823: f64, t51730: f64, t6036: f64, t6052: f64, t6069: f64, t6085: f64, t11185: f64, t18262: f64, t14913: f64, t4785: f64, t18266: f64, t43964: f64, t18265: f64, t44075: f64, t44077: f64, t5988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63591, t63594, t63611) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3063(t18834, t3315, t1117, t3313, t18258, t3307, t1147, t18710, t3400, t6063, t1157, t15121, t15133, t1695, t18899, t3396, t3404, t44300, t4835, t4858, t51366, t6056, t63563, t63566, t63568, t63571, t63574, t63576, t63579, t63582, t63585, t63587);
        let t63665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3064(t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63323);
        let t63679 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3065(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let t63692 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3066(t43780, t43782, t43816, t43942, t50952, t50954, t63355, t63359, t63361, t63365, t63370, t63374);
        let t63706 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3067(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
        let (t63709, t63714, t63715) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3068(t449, t63665, t63679, t63692, t63706, t11275, t11277, t3265, t6020, t11297, t11350, t11352, t11356, t11361, t11415, t1155, t15117, t15153, t15156, t15207, t18606, t18609, t18612, t18616, t18647, t18650, t18651, t18786, t3333, t3351, t3357, t3376, t44172, t44177, t44179, t44202, t44205, t4802, t4823, t51730, t6036, t6052, t6069, t6085);
        let (t63717, t63720, t63722, t63725, t63729) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3069(t11185, t18262, t14913, t3313, t4785, t18266, t43964, t11275, t18265, t3307, t3265, t44075, t44077, t5988);
    (t63591, t63594, t63611, t63709, t63714, t63715, t63717, t63720, t63722, t63725, t63729)
}
