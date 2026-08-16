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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1447;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1448;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1449;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1450;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1451;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1452;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1453;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta386<F: Float>(t3447: F, t3451: F, t44510: F, t11496: F, t3448: F, t11502: F, t1184: F, t15418: F, t11571: F, t3469: F, t4899: F, t11570: F, t9288: F, t3450: F, t9258: F, t11584: F, t11593: F, t1174: F, t24705: F, t3449: F, t43719: F, t43723: F, t44499: F, t44502: F, t44504: F, t44506: F, t4908: F, t4934: F, t3475: F, t11545: F, t135: F, t11548: F, t43791: F, t461: F, t3439: F, t698: F, t3442: F, t11588: F, t1176: F, t697: F, t11579: F, t11589: F, t11168: F, t15402: F, t11159: F, t15419: F, t11546: F, t11575: F, t3440: F, t3441: F, t39097: F, t39103: F, t43715: F, t4900: F, t11153: F, t460: F, t3242: F, t405: F, t974: F, t43763: F, t11509: F, t15281: F, t11525: F, t3431: F, t2402: F, t1179: F, t11529: F, t3460: F, t3456: F, t11516: F, t11547: F, t11569: F, t1177: F, t1178: F, t15395: F, t3455: F, t39110: F, t43711: F, t43732: F, t44493: F, t3630: F, t3493: F, t491: F, t11720: F, t1235: F, t10469: F, t1190: F, t11887: F) -> (F, F, F, F, F, F, F, F) {
        let (t44512, t44517, t44521, t44527, t44529, t44536) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1447::<F>(t3447, t3451, t44510, t11496, t3448, t11502, t1184, t15418, t11571, t3469, t4899, t11570, t9288);
        let t44547 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1448::<F>(t3450, t9258, t11571, t11584, t11593, t1174, t24705, t3447, t3449, t3451, t3469, t43719, t43723, t44499, t44502, t44504, t44506, t44512, t44517, t44521, t44527, t44529, t44536, t4908, t4934);
        let (t44558, t44564, t44566, t44573, t44579) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1449::<F>(t3475, t4899, t11545, t135, t11548, t1174, t43791, t461, t3439, t698, t3442, t11588);
        let (t44581, t44586, t44589, t44592, t44595) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1450::<F>(t3447, t3451, t44579, t1176, t697, t1184, t11579, t11589, t11168, t15402, t11159, t15419);
        let t44600 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1451::<F>(t11546, t11571, t11575, t11579, t11584, t11593, t1174, t3440, t3441, t3447, t39097, t39103, t43715, t44558, t44564, t44566, t44573, t44581, t44586, t44589, t44592, t44595, t4900);
        let (t44602, t44608, t44620, t44621, t44622, t44628) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1452::<F>(t11584, t11589, t3447, t11153, t460, t9288, t3242, t405, t974, t43763, t461, t11509, t1174, t15281);
        let t44655 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1453::<F>(t11525, t1174, t3431, t1176, t2402, t1179, t11529, t3460, t3456, t11516, t11547, t11569, t1177, t1178, t15395, t3440, t3447, t3455, t39097, t39103, t39110, t43711, t43732, t44602, t44608, t44621, t44622, t44628, t4900);
        let (t44657, t44662, t44668, t44669, t44673, t44690, t44691) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1454::<F>(t44493, t44547, t44600, t44655, t3630, t3493, t491, t11720, t1235, t10469, t1190, t11887);
    (t44620, t44657, t44662, t44668, t44669, t44673, t44690, t44691)
}
