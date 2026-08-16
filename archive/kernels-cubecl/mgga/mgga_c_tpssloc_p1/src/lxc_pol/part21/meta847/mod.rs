//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta847 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3063;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3064;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3065;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3066;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3067;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3068;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta847<F: Float>(t18834: F, t3315: F, t1117: F, t3313: F, t18258: F, t3307: F, t1147: F, t18710: F, t3400: F, t6063: F, t1157: F, t15121: F, t15133: F, t1695: F, t18899: F, t3396: F, t3404: F, t44300: F, t4835: F, t4858: F, t51366: F, t6056: F, t63563: F, t63566: F, t63568: F, t63571: F, t63574: F, t63576: F, t63579: F, t63582: F, t63585: F, t63587: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63323: F, t43748: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t63327: F, t63330: F, t63332: F, t63334: F, t63336: F, t43780: F, t43782: F, t43816: F, t43942: F, t50952: F, t50954: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t449: F, t11275: F, t11277: F, t3265: F, t6020: F, t11297: F, t11350: F, t11352: F, t11356: F, t11361: F, t11415: F, t1155: F, t15117: F, t15153: F, t15156: F, t15207: F, t18606: F, t18609: F, t18612: F, t18616: F, t18647: F, t18650: F, t18651: F, t18786: F, t3333: F, t3351: F, t3357: F, t3376: F, t44172: F, t44177: F, t44179: F, t44202: F, t44205: F, t4802: F, t4823: F, t51730: F, t6036: F, t6052: F, t6069: F, t6085: F, t11185: F, t18262: F, t14913: F, t4785: F, t18266: F, t43964: F, t18265: F, t44075: F, t44077: F, t5988: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t63591, t63594, t63611) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3063::<F>(t18834, t3315, t1117, t3313, t18258, t3307, t1147, t18710, t3400, t6063, t1157, t15121, t15133, t1695, t18899, t3396, t3404, t44300, t4835, t4858, t51366, t6056, t63563, t63566, t63568, t63571, t63574, t63576, t63579, t63582, t63585, t63587);
        let t63665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3064::<F>(t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63323);
        let t63679 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3065::<F>(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let t63692 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3066::<F>(t43780, t43782, t43816, t43942, t50952, t50954, t63355, t63359, t63361, t63365, t63370, t63374);
        let t63706 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3067::<F>(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
        let (t63709, t63714, t63715) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3068::<F>(t449, t63665, t63679, t63692, t63706, t11275, t11277, t3265, t6020, t11297, t11350, t11352, t11356, t11361, t11415, t1155, t15117, t15153, t15156, t15207, t18606, t18609, t18612, t18616, t18647, t18650, t18651, t18786, t3333, t3351, t3357, t3376, t44172, t44177, t44179, t44202, t44205, t4802, t4823, t51730, t6036, t6052, t6069, t6085);
        let (t63717, t63720, t63722, t63725, t63729) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3069::<F>(t11185, t18262, t14913, t3313, t4785, t18266, t43964, t11275, t18265, t3307, t3265, t44075, t44077, t5988);
    (t63591, t63594, t63611, t63709, t63714, t63715, t63717, t63720, t63722, t63725, t63729)
}
