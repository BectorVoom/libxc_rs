//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1354;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1355;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1356;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta463(t136: f64, t2826: f64, t76608: f64, t76612: f64, t908: f64, t76616: f64, t76620: f64, t43002: f64, t48103: f64, t60168: f64, t60173: f64, t60204: f64, t68452: f64, t68454: f64, t76903: f64, t13769: f64, t17794: f64, t17804: f64, t2986: f64, t340: f64, t343: f64, t4510: f64, t4531: f64, t61310: f64, t61313: f64, t69548: f64, t69647: f64, t69683: f64, t69686: f64, t69691: f64, t69699: f64, t69727: f64, t69739: f64, t69746: f64, t76593: f64, t76901: f64, t973: f64, t974: f64, t10214: f64, t10217: f64, t10278: f64, t1597: f64, t21444: f64, t2979: f64, t2980: f64, t42976: f64, t4546: f64, t48336: f64, t48397: f64, t61408: f64, t61489: f64, t61597: f64, t61600: f64, t69796: f64, t69801: f64, t69806: f64, t75836: f64, t75847: f64, t977: f64, t76829: f64, t76865: f64, t225: f64, t76634: f64, t76636: f64, t76641: f64, t76643: f64, t76647: f64, t76652: f64, t76654: f64, t76657: f64, t76659: f64, t76661: f64, t76663: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t76906, t76909, t76912, t76915, t76922) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1354(t136, t2826, t76608, t76612, t908, t76616, t76620, t43002, t48103, t60168, t60173, t60204, t68452, t68454, t76903);
        let t76943 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1355(t13769, t17794, t17804, t2986, t340, t343, t4510, t4531, t61310, t61313, t69548, t69647, t69683, t69686, t69691, t69699, t69727, t69739, t69746, t76593, t76901, t76922, t973, t974);
        let t76974 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1356(t10214, t10217, t10278, t1597, t21444, t2979, t2980, t343, t42976, t4546, t48336, t48397, t61408, t61489, t61597, t61600, t69796, t69801, t69806, t75836, t75847, t973, t977);
        let (t76976, t76977, t76995) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1357(t76829, t76865, t76943, t76974, t225, t76634, t76636, t76641, t76643, t76647, t76652, t76654, t76657, t76659, t76661, t76663);
    (t76906, t76909, t76912, t76915, t76976, t76977, t76995)
}
