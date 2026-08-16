//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta714 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2317;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2318;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2319;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta714(t21061: f64, t225: f64, t21036: f64, t20856: f64, t252: f64, t1519: f64, t5584: f64, t20852: f64, t13176: f64, t13433: f64, t13453: f64, t16673: f64, t16756: f64, t16758: f64, t16762: f64, t16817: f64, t16825: f64, t16830: f64, t16935: f64, t17034: f64, t21025: f64, t4166: f64, t4182: f64, t4281: f64, t4296: f64, t5612: f64, t5645: f64, t5651: f64, t58313: f64, t812: f64, t5611: f64, t21013: f64, t814: f64, t20937: f64, t68: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40673: f64, t40679: f64, t46138: f64, t67044: f64, t67086: f64, t67087: f64, t67088: f64, t67089: f64, t67090: f64, t67095: f64, t67096: f64, t39373: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t40685: f64, t40708: f64, t40714: f64, t40716: f64, t46207: f64, t67097: f64, t67100: f64, t67104: f64, t67105: f64, t67127: f64, t67132: f64, t67133: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t40721: f64, t40732: f64, t46218: f64, t46235: f64, t46237: f64, t67137: f64, t67141: f64, t67146: f64, t67147: f64, t67153: f64, t67158: f64, t67159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67339, t67344, t67350, t67358, t67392, t67403) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2317(t21061, t225, t21036, t20856, t252, t1519, t5584, t20852, t13176, t13433, t13453, t16673, t16756, t16758, t16762, t16817, t16825, t16830, t16935, t17034, t21025, t4166, t4182, t4281, t4296, t5612, t5645, t5651, t58313, t812);
        let (t67405, t67429, t67441, t67448) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2318(t1519, t5611, t21013, t814, t20937, t68, t39249, t39256, t39309, t39312, t39316, t39320, t40673, t40679, t46138, t67044, t67086, t67087, t67088, t67089, t67090, t67095, t67096);
        let t67449 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2319(t39373, t39397, t39400, t39408, t39411, t40685, t40708, t40714, t40716, t46207, t67097, t67100, t67104, t67105, t67127, t67132, t67133);
        let t67451 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2320(t39463, t39468, t39472, t39476, t39483, t40721, t40732, t46218, t46235, t46237, t67137, t67141, t67146, t67147, t67153, t67158, t67159);
    (t67339, t67344, t67350, t67358, t67392, t67403, t67405, t67429, t67441, t67448, t67449, t67451)
}
