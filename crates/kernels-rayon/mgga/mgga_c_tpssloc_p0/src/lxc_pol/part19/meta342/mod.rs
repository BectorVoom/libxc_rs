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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta342(t41083: f64, t789: f64, t41011: f64, t9561: f64, t154: f64, t1891: f64, t205: f64, t792: f64, t9558: f64, t118: f64, t794: f64, t9458: f64, t2576: f64, t9516: f64, t207: f64, t40394: f64, t40399: f64, t210: f64, t214: f64, t2571: f64, t40848: f64, t40972: f64, t40977: f64, t41142: f64, t41144: f64, t41149: f64, t41151: f64, t41155: f64, t787: f64, t2582: f64, t9541: f64, t786: f64, t9580: f64, t2578: f64, t9546: f64, t9555: f64, t2573: f64, t41008: f64, t2566: f64, t2570: f64, t9551: f64, t2588: f64, t40341: f64, t12998: f64, t2553: f64, t686: f64, t9524: f64, t13012: f64, t9566: f64, t215: f64, t39933: f64, t40344: f64, t795: f64, t116: f64, t9534: f64, t133: f64, t6600: f64, t776: f64, t13005: f64, t213: f64, t221: f64, t2379: f64, t4127: f64, t225: f64, t2639: f64, t9960: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40627: f64, t40663: f64, t40668: f64, t40671: f64, t40674: f64, t39373: f64, t39397: f64, t39400: f64, t40677: f64, t40679: f64, t40681: f64, t40683: f64, t40685: f64, t40688: f64, t40690: f64, t40708: f64, t39408: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t40711: f64, t40714: f64, t40716: f64, t40721: f64, t40723: f64, t39483: f64, t40727: f64, t40730: f64, t40732: f64, t40734: f64, t40737: f64, t40739: f64, t40741: f64, t40743: f64, t40746: f64, t40748: f64, t40750: f64, t39529: f64, t40755: f64, t40760: f64, t40762: f64, t40764: f64, t40766: f64, t40768: f64, t40777: f64, t40779: f64, t40782: f64, t40784: f64, t39549: f64, t40790: f64, t40793: f64, t40795: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t40805: f64, t40807: f64, t40809: f64, t40811: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41156, t41158, t41161, t41173) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1219(t41083, t789, t41011, t9561, t154, t1891, t205, t792, t9558, t118, t794, t9458);
        let t41186 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1220(t118, t2576, t794, t9516, t207, t40394, t40399, t210, t214, t2571, t40848, t40972, t40977, t41142, t41144, t41149, t41151, t41155, t41156, t41158, t41161, t41173, t787);
        let (t41187, t41190, t41192, t41194, t41197) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1221(t2582, t9541, t786, t9580, t2578, t9546, t9555, t2573, t41008, t2566, t2570, t9551);
        let (t41200, t41203, t41205, t41209, t41212) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1222(t2588, t40341, t12998, t2553, t686, t9524, t13012, t9566, t207, t215, t39933, t40344, t795);
        let t41229 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1223(t116, t786, t9534, t133, t6600, t776, t13005, t213, t221, t2379, t2553, t41187, t41190, t41192, t41194, t41197, t41200, t41203, t41205, t41209, t41212, t4127, t9516);
        let (t41230, t41231, t41237, t41241) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1224(t41186, t41229, t225, t2639, t9960, t39249, t39256, t39309, t39312, t39316, t39320, t40627, t40663, t40668, t40671, t40674);
        let (t41242, t41244) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1225(t39373, t39397, t39400, t40677, t40679, t40681, t40683, t40685, t40688, t40690, t40708, t39408, t39411, t39463, t39468, t39472, t39476, t40711, t40714, t40716, t40721, t40723);
        let t41245 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1226(t39483, t40727, t40730, t40732, t40734, t40737, t40739, t40741, t40743, t40746, t40748, t40750);
        let (t41248, t41249) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1227(t39529, t40755, t40760, t40762, t40764, t40766, t40768, t40777, t40779, t40782, t40784, t39549, t40790, t40793, t40795, t40797, t40799, t40801, t40803, t40805, t40807, t40809, t40811);
    (t41161, t41230, t41231, t41237, t41241, t41242, t41244, t41245, t41248, t41249)
}
