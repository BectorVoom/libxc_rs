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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2971;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2972;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2973;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2974;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2975;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2976;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2977;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta921(t18900: f64, t4719: f64, t78328: f64, t78332: f64, t78335: f64, t78339: f64, t78342: f64, t78703: f64, t78706: f64, t78709: f64, t78712: f64, t78715: f64, t78303: f64, t78305: f64, t78307: f64, t78309: f64, t78311: f64, t78313: f64, t78315: f64, t78319: f64, t78322: f64, t78325: f64, t78682: f64, t78683: f64, t78699: f64, t4772: f64, t6244: f64, t1041: f64, t1042: f64, t1045: f64, t1062: f64, t11656: f64, t11703: f64, t15716: f64, t15728: f64, t16089: f64, t19707: f64, t23643: f64, t23823: f64, t23859: f64, t23966: f64, t247: f64, t2852: f64, t3116: f64, t3124: f64, t373: f64, t4181: f64, t42879: f64, t42914: f64, t4839: f64, t55202: f64, t6308: f64, t65347: f64, t65357: f64, t65359: f64, t65376: f64, t65431: f64, t65444: f64, t65446: f64, t66047: f64, t67501: f64, t78676: f64, t1063: f64, t23470: f64, t42534: f64, t20050: f64, t4834: f64, t23843: f64, t3172: f64, t5819: f64, t22671: f64, t606: f64, t22688: f64, t4186: f64, t11922: f64, t11927: f64, t23838: f64, t23998: f64, t3115: f64, t11672: f64, t15618: f64, t15935: f64, t16208: f64, t1675: f64, t19878: f64, t19944: f64, t20079: f64, t23839: f64, t23848: f64, t23917: f64, t3127: f64, t43297: f64, t4801: f64, t4806: f64, t51958: f64, t51963: f64, t66784: f64, t78570: f64, t23640: f64, t42871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78717, t78718) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2971(t18900, t4719, t78328, t78332, t78335, t78339, t78342, t78703, t78706, t78709, t78712, t78715);
        let t78721 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2972(t78303, t78305, t78307, t78309, t78311, t78313, t78315, t78319, t78322, t78325, t78682, t78683, t78699, t78718);
        let (t78740, t78745) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2973(t4772, t6244, t1041, t1042, t1045, t1062, t11656, t11703, t15716, t15728, t16089, t19707, t23643, t23823, t23859, t23966, t247, t2852, t3116, t3124, t373, t4181, t42879, t42914, t4839, t55202, t6308, t65347, t65357, t65359, t65376, t65431, t65444, t65446, t66047, t67501, t78676, t78721);
        let (t78750, t78756, t78763, t78765, t78770) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2974(t1063, t23470, t247, t42534, t20050, t4834, t23843, t3172, t4772, t5819, t22671, t606);
        let t78785 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2975(t22688, t606);
        let t78790 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2976(t4186, t5819);
        let t78807 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2977(t11922, t11927, t23838, t23998, t3115, t1042, t1063, t11656, t11672, t15618, t15935, t16208, t1675, t19878, t19944, t20079, t23839, t23848, t23917, t3127, t43297, t4801, t4806, t51958, t51963, t66784, t78570, t78750, t78756, t78763, t78765, t78770, t78785, t78790);
        let t78812 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2978(t23640, t42871);
    (t78717, t78721, t78740, t78745, t78765, t78770, t78785, t78790, t78807, t78812)
}
