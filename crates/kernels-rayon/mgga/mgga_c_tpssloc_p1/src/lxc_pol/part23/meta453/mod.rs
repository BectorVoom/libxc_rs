//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1305;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1306;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1307;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1308;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1309;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta453(t40: f64, t5499: f64, t57973: f64, t46369: f64, t46371: f64, t16637: f64, t20217: f64, t2291: f64, t4104: f64, t5398: f64, t75: f64, t75836: f64, t75847: f64, t75912: f64, t767: f64, zeta_threshold: f64, t52: f64, t16649: f64, t2298: f64, t4111: f64, t771: f64, t78: f64, t5611: f64, t2632: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t75839: f64, t75840: f64, t75844: f64, t75845: f64, t75846: f64, t75850: f64, t75851: f64, t39316: f64, t39320: f64, t39373: f64, t39397: f64, t39400: f64, t40679: f64, t40685: f64, t40708: f64, t75854: f64, t75855: f64, t75856: f64, t39408: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t40714: f64, t40716: f64, t40721: f64, t75864: f64, t75865: f64, t39483: f64, t40732: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64, t75872: f64, t75874: f64, t75884: f64, t75885: f64, t75886: f64, t75887: f64, t39529: f64, t40764: f64, t40766: f64, t40779: f64, t40784: f64, t75894: f64, t75895: f64, t75900: f64, t75901: f64, t75932: f64, t75933: f64, t39549: f64, t39563: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t75939: f64, t75940: f64, t75941: f64, t75942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75950, t75951, t75952, t75964) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1305(t40, t5499, t57973, t46369, t46371, t16637, t20217, t2291, t4104, t5398, t75, t75836, t75847, t75912, t767, zeta_threshold);
        let t75978 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1306(t52, t16649, t20217, t2298, t4111, t5398, t75836, t75847, t75912, t771, t78, t75964, zeta_threshold);
        let (t76001, t76002, t76006) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1307(t5611, t2632, t39249, t39256, t39309, t39312, t75839, t75840, t75844, t75845, t75846, t75850, t75851);
        let (t76007, t76009) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1308(t39316, t39320, t39373, t39397, t39400, t40679, t40685, t40708, t75854, t75855, t75856, t39408, t39411, t39463, t39468, t39472, t39476, t40714, t40716, t40721, t75864, t75865);
        let t76010 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1309(t39483, t40732, t40741, t40743, t40748, t40760, t75872, t75874, t75884, t75885, t75886, t75887);
        let (t76013, t76014) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1310(t39529, t40764, t40766, t40779, t40784, t75894, t75895, t75900, t75901, t75932, t75933, t39549, t39563, t40790, t40793, t40797, t40799, t40801, t40803, t75939, t75940, t75941, t75942);
    (t75950, t75951, t75952, t75978, t76001, t76002, t76006, t76007, t76009, t76010, t76013, t76014)
}
