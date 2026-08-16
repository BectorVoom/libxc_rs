//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1305;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1306;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1307;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1308;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1309;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta453<F: Float>(t40: F, t5499: F, t57973: F, t46369: F, t46371: F, t16637: F, t20217: F, t2291: F, t4104: F, t5398: F, t75: F, t75836: F, t75847: F, t75912: F, t767: F, zeta_threshold: F, t52: F, t16649: F, t2298: F, t4111: F, t771: F, t78: F, t5611: F, t2632: F, t39249: F, t39256: F, t39309: F, t39312: F, t75839: F, t75840: F, t75844: F, t75845: F, t75846: F, t75850: F, t75851: F, t39316: F, t39320: F, t39373: F, t39397: F, t39400: F, t40679: F, t40685: F, t40708: F, t75854: F, t75855: F, t75856: F, t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t40714: F, t40716: F, t40721: F, t75864: F, t75865: F, t39483: F, t40732: F, t40741: F, t40743: F, t40748: F, t40760: F, t75872: F, t75874: F, t75884: F, t75885: F, t75886: F, t75887: F, t39529: F, t40764: F, t40766: F, t40779: F, t40784: F, t75894: F, t75895: F, t75900: F, t75901: F, t75932: F, t75933: F, t39549: F, t39563: F, t40790: F, t40793: F, t40797: F, t40799: F, t40801: F, t40803: F, t75939: F, t75940: F, t75941: F, t75942: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t75950, t75951, t75952, t75964) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1305::<F>(t40, t5499, t57973, t46369, t46371, t16637, t20217, t2291, t4104, t5398, t75, t75836, t75847, t75912, t767, zeta_threshold);
        let t75978 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1306::<F>(t52, t16649, t20217, t2298, t4111, t5398, t75836, t75847, t75912, t771, t78, t75964, zeta_threshold);
        let (t76001, t76002, t76006) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1307::<F>(t5611, t2632, t39249, t39256, t39309, t39312, t75839, t75840, t75844, t75845, t75846, t75850, t75851);
        let (t76007, t76009) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1308::<F>(t39316, t39320, t39373, t39397, t39400, t40679, t40685, t40708, t75854, t75855, t75856, t39408, t39411, t39463, t39468, t39472, t39476, t40714, t40716, t40721, t75864, t75865);
        let t76010 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1309::<F>(t39483, t40732, t40741, t40743, t40748, t40760, t75872, t75874, t75884, t75885, t75886, t75887);
        let (t76013, t76014) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1310::<F>(t39529, t40764, t40766, t40779, t40784, t75894, t75895, t75900, t75901, t75932, t75933, t39549, t39563, t40790, t40793, t40797, t40799, t40801, t40803, t75939, t75940, t75941, t75942);
    (t75950, t75951, t75952, t75978, t76001, t76002, t76006, t76007, t76009, t76010, t76013, t76014)
}
