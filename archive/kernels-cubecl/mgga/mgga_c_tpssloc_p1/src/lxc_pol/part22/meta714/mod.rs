//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta714 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2317;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2318;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2319;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta714<F: Float>(t21061: F, t225: F, t21036: F, t20856: F, t252: F, t1519: F, t5584: F, t20852: F, t13176: F, t13433: F, t13453: F, t16673: F, t16756: F, t16758: F, t16762: F, t16817: F, t16825: F, t16830: F, t16935: F, t17034: F, t21025: F, t4166: F, t4182: F, t4281: F, t4296: F, t5612: F, t5645: F, t5651: F, t58313: F, t812: F, t5611: F, t21013: F, t814: F, t20937: F, t68: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40673: F, t40679: F, t46138: F, t67044: F, t67086: F, t67087: F, t67088: F, t67089: F, t67090: F, t67095: F, t67096: F, t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40685: F, t40708: F, t40714: F, t40716: F, t46207: F, t67097: F, t67100: F, t67104: F, t67105: F, t67127: F, t67132: F, t67133: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t40721: F, t40732: F, t46218: F, t46235: F, t46237: F, t67137: F, t67141: F, t67146: F, t67147: F, t67153: F, t67158: F, t67159: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t67339, t67344, t67350, t67358, t67392, t67403) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2317::<F>(t21061, t225, t21036, t20856, t252, t1519, t5584, t20852, t13176, t13433, t13453, t16673, t16756, t16758, t16762, t16817, t16825, t16830, t16935, t17034, t21025, t4166, t4182, t4281, t4296, t5612, t5645, t5651, t58313, t812);
        let (t67405, t67429, t67441, t67448) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2318::<F>(t1519, t5611, t21013, t814, t20937, t68, t39249, t39256, t39309, t39312, t39316, t39320, t40673, t40679, t46138, t67044, t67086, t67087, t67088, t67089, t67090, t67095, t67096);
        let t67449 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2319::<F>(t39373, t39397, t39400, t39408, t39411, t40685, t40708, t40714, t40716, t46207, t67097, t67100, t67104, t67105, t67127, t67132, t67133);
        let t67451 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2320::<F>(t39463, t39468, t39472, t39476, t39483, t40721, t40732, t46218, t46235, t46237, t67137, t67141, t67146, t67147, t67153, t67158, t67159);
    (t67339, t67344, t67350, t67358, t67392, t67403, t67405, t67429, t67441, t67448, t67449, t67451)
}
