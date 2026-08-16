//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta342 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1219;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1220;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1221;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1222;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1223;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1224;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1225;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1226;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta342<F: Float>(t41083: F, t789: F, t41011: F, t9561: F, t154: F, t1891: F, t205: F, t792: F, t9558: F, t118: F, t794: F, t9458: F, t2576: F, t9516: F, t207: F, t40394: F, t40399: F, t210: F, t214: F, t2571: F, t40848: F, t40972: F, t40977: F, t41142: F, t41144: F, t41149: F, t41151: F, t41155: F, t787: F, t2582: F, t9541: F, t786: F, t9580: F, t2578: F, t9546: F, t9555: F, t2573: F, t41008: F, t2566: F, t2570: F, t9551: F, t2588: F, t40341: F, t12998: F, t2553: F, t686: F, t9524: F, t13012: F, t9566: F, t215: F, t39933: F, t40344: F, t795: F, t116: F, t9534: F, t133: F, t6600: F, t776: F, t13005: F, t213: F, t221: F, t2379: F, t4127: F, t225: F, t2639: F, t9960: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40627: F, t40663: F, t40668: F, t40671: F, t40674: F, t39373: F, t39397: F, t39400: F, t40677: F, t40679: F, t40681: F, t40683: F, t40685: F, t40688: F, t40690: F, t40708: F, t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t40711: F, t40714: F, t40716: F, t40721: F, t40723: F, t39483: F, t40727: F, t40730: F, t40732: F, t40734: F, t40737: F, t40739: F, t40741: F, t40743: F, t40746: F, t40748: F, t40750: F, t39529: F, t40755: F, t40760: F, t40762: F, t40764: F, t40766: F, t40768: F, t40777: F, t40779: F, t40782: F, t40784: F, t39549: F, t40790: F, t40793: F, t40795: F, t40797: F, t40799: F, t40801: F, t40803: F, t40805: F, t40807: F, t40809: F, t40811: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41156, t41158, t41161, t41173) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1219::<F>(t41083, t789, t41011, t9561, t154, t1891, t205, t792, t9558, t118, t794, t9458);
        let t41186 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1220::<F>(t118, t2576, t794, t9516, t207, t40394, t40399, t210, t214, t2571, t40848, t40972, t40977, t41142, t41144, t41149, t41151, t41155, t41156, t41158, t41161, t41173, t787);
        let (t41187, t41190, t41192, t41194, t41197) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1221::<F>(t2582, t9541, t786, t9580, t2578, t9546, t9555, t2573, t41008, t2566, t2570, t9551);
        let (t41200, t41203, t41205, t41209, t41212) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1222::<F>(t2588, t40341, t12998, t2553, t686, t9524, t13012, t9566, t207, t215, t39933, t40344, t795);
        let t41229 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1223::<F>(t116, t786, t9534, t133, t6600, t776, t13005, t213, t221, t2379, t2553, t41187, t41190, t41192, t41194, t41197, t41200, t41203, t41205, t41209, t41212, t4127, t9516);
        let (t41230, t41231, t41237, t41241) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1224::<F>(t41186, t41229, t225, t2639, t9960, t39249, t39256, t39309, t39312, t39316, t39320, t40627, t40663, t40668, t40671, t40674);
        let (t41242, t41244) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1225::<F>(t39373, t39397, t39400, t40677, t40679, t40681, t40683, t40685, t40688, t40690, t40708, t39408, t39411, t39463, t39468, t39472, t39476, t40711, t40714, t40716, t40721, t40723);
        let t41245 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1226::<F>(t39483, t40727, t40730, t40732, t40734, t40737, t40739, t40741, t40743, t40746, t40748, t40750);
        let (t41248, t41249) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1227::<F>(t39529, t40755, t40760, t40762, t40764, t40766, t40768, t40777, t40779, t40782, t40784, t39549, t40790, t40793, t40795, t40797, t40799, t40801, t40803, t40805, t40807, t40809, t40811);
    (t41161, t41230, t41231, t41237, t41241, t41242, t41244, t41245, t41248, t41249)
}
