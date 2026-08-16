//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta921 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2971;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2972;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2973;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2974;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2975;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2976;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2977;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta921<F: Float>(t18900: F, t4719: F, t78328: F, t78332: F, t78335: F, t78339: F, t78342: F, t78703: F, t78706: F, t78709: F, t78712: F, t78715: F, t78303: F, t78305: F, t78307: F, t78309: F, t78311: F, t78313: F, t78315: F, t78319: F, t78322: F, t78325: F, t78682: F, t78683: F, t78699: F, t4772: F, t6244: F, t1041: F, t1042: F, t1045: F, t1062: F, t11656: F, t11703: F, t15716: F, t15728: F, t16089: F, t19707: F, t23643: F, t23823: F, t23859: F, t23966: F, t247: F, t2852: F, t3116: F, t3124: F, t373: F, t4181: F, t42879: F, t42914: F, t4839: F, t55202: F, t6308: F, t65347: F, t65357: F, t65359: F, t65376: F, t65431: F, t65444: F, t65446: F, t66047: F, t67501: F, t78676: F, t1063: F, t23470: F, t42534: F, t20050: F, t4834: F, t23843: F, t3172: F, t5819: F, t22671: F, t606: F, t22688: F, t4186: F, t11922: F, t11927: F, t23838: F, t23998: F, t3115: F, t11672: F, t15618: F, t15935: F, t16208: F, t1675: F, t19878: F, t19944: F, t20079: F, t23839: F, t23848: F, t23917: F, t3127: F, t43297: F, t4801: F, t4806: F, t51958: F, t51963: F, t66784: F, t78570: F, t23640: F, t42871: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t78717, t78718) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2971::<F>(t18900, t4719, t78328, t78332, t78335, t78339, t78342, t78703, t78706, t78709, t78712, t78715);
        let t78721 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2972::<F>(t78303, t78305, t78307, t78309, t78311, t78313, t78315, t78319, t78322, t78325, t78682, t78683, t78699, t78718);
        let (t78740, t78745) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2973::<F>(t4772, t6244, t1041, t1042, t1045, t1062, t11656, t11703, t15716, t15728, t16089, t19707, t23643, t23823, t23859, t23966, t247, t2852, t3116, t3124, t373, t4181, t42879, t42914, t4839, t55202, t6308, t65347, t65357, t65359, t65376, t65431, t65444, t65446, t66047, t67501, t78676, t78721);
        let (t78750, t78756, t78763, t78765, t78770) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2974::<F>(t1063, t23470, t247, t42534, t20050, t4834, t23843, t3172, t4772, t5819, t22671, t606);
        let t78785 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2975::<F>(t22688, t606);
        let t78790 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2976::<F>(t4186, t5819);
        let t78807 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2977::<F>(t11922, t11927, t23838, t23998, t3115, t1042, t1063, t11656, t11672, t15618, t15935, t16208, t1675, t19878, t19944, t20079, t23839, t23848, t23917, t3127, t43297, t4801, t4806, t51958, t51963, t66784, t78570, t78750, t78756, t78763, t78765, t78770, t78785, t78790);
        let t78812 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2978::<F>(t23640, t42871);
    (t78717, t78721, t78740, t78745, t78765, t78770, t78785, t78790, t78807, t78812)
}
