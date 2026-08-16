//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta386 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1447;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1448;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1449;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1450;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1451;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1452;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1453;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta386(t3447: f64, t3451: f64, t44510: f64, t11496: f64, t3448: f64, t11502: f64, t1184: f64, t15418: f64, t11571: f64, t3469: f64, t4899: f64, t11570: f64, t9288: f64, t3450: f64, t9258: f64, t11584: f64, t11593: f64, t1174: f64, t24705: f64, t3449: f64, t43719: f64, t43723: f64, t44499: f64, t44502: f64, t44504: f64, t44506: f64, t4908: f64, t4934: f64, t3475: f64, t11545: f64, t135: f64, t11548: f64, t43791: f64, t461: f64, t3439: f64, t698: f64, t3442: f64, t11588: f64, t1176: f64, t697: f64, t11579: f64, t11589: f64, t11168: f64, t15402: f64, t11159: f64, t15419: f64, t11546: f64, t11575: f64, t3440: f64, t3441: f64, t39097: f64, t39103: f64, t43715: f64, t4900: f64, t11153: f64, t460: f64, t3242: f64, t405: f64, t974: f64, t43763: f64, t11509: f64, t15281: f64, t11525: f64, t3431: f64, t2402: f64, t1179: f64, t11529: f64, t3460: f64, t3456: f64, t11516: f64, t11547: f64, t11569: f64, t1177: f64, t1178: f64, t15395: f64, t3455: f64, t39110: f64, t43711: f64, t43732: f64, t44493: f64, t3630: f64, t3493: f64, t491: f64, t11720: f64, t1235: f64, t10469: f64, t1190: f64, t11887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44512, t44517, t44521, t44527, t44529, t44536) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1447(t3447, t3451, t44510, t11496, t3448, t11502, t1184, t15418, t11571, t3469, t4899, t11570, t9288);
        let t44547 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1448(t3450, t9258, t11571, t11584, t11593, t1174, t24705, t3447, t3449, t3451, t3469, t43719, t43723, t44499, t44502, t44504, t44506, t44512, t44517, t44521, t44527, t44529, t44536, t4908, t4934);
        let (t44558, t44564, t44566, t44573, t44579) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1449(t3475, t4899, t11545, t135, t11548, t1174, t43791, t461, t3439, t698, t3442, t11588);
        let (t44581, t44586, t44589, t44592, t44595) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1450(t3447, t3451, t44579, t1176, t697, t1184, t11579, t11589, t11168, t15402, t11159, t15419);
        let t44600 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1451(t11546, t11571, t11575, t11579, t11584, t11593, t1174, t3440, t3441, t3447, t39097, t39103, t43715, t44558, t44564, t44566, t44573, t44581, t44586, t44589, t44592, t44595, t4900);
        let (t44602, t44608, t44620, t44621, t44622, t44628) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1452(t11584, t11589, t3447, t11153, t460, t9288, t3242, t405, t974, t43763, t461, t11509, t1174, t15281);
        let t44655 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1453(t11525, t1174, t3431, t1176, t2402, t1179, t11529, t3460, t3456, t11516, t11547, t11569, t1177, t1178, t15395, t3440, t3447, t3455, t39097, t39103, t39110, t43711, t43732, t44602, t44608, t44621, t44622, t44628, t4900);
        let (t44657, t44662, t44668, t44669, t44673, t44690, t44691) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1454(t44493, t44547, t44600, t44655, t3630, t3493, t491, t11720, t1235, t10469, t1190, t11887);
    (t44620, t44657, t44662, t44668, t44669, t44673, t44690, t44691)
}
