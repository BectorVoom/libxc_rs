//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1348;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1349;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1350;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta461(t21303: f64, t49274: f64, t10704: f64, t42028: f64, t76644: f64, t21239: f64, t4488: f64, t959: f64, t5950: f64, t5919: f64, t5943: f64, t10165: f64, t1052: f64, t1634: f64, t1635: f64, t17588: f64, t18074: f64, t21662: f64, t21663: f64, t21677: f64, t21692: f64, t3174: f64, t388: f64, t43604: f64, t4557: f64, t4660: f64, t5848: f64, t5914: f64, t5920: f64, t69871: f64, t70978: f64, t70980: f64, t5866: f64, t5872: f64, t1021: f64, t10408: f64, t1041: f64, t10413: f64, t10482: f64, t1622: f64, t17177: f64, t17607: f64, t17923: f64, t18030: f64, t21393: f64, t21398: f64, t21516: f64, t248: f64, t28651: f64, t3039: f64, t3070: f64, t3071: f64, t360: f64, t43291: f64, t43292: f64, t43385: f64, t43399: f64, t4644: f64, t48570: f64, t50265: f64, t5857: f64, t5861: f64, t5869: f64, t5875: f64, t61663: f64, t61736: f64, t70122: f64, t70132: f64, t70138: f64, t70153: f64, t76572: f64, t5836: f64, t5842: f64, t1539: f64, t17800: f64, t17817: f64, t17863: f64, t2986: f64, t2994: f64, t340: f64, t343: f64, t42861: f64, t42862: f64, t4531: f64, t4546: f64, t61365: f64, t69487: f64, t69503: f64, t69515: f64, t69540: f64, t7577: f64, t75836: f64, t75847: f64, t75912: f64, t973: f64, t974: f64, t977: f64, t978: f64, t13798: f64, t17794: f64, t17804: f64, t42817: f64, t4510: f64, t4514: f64, t4518: f64, t48221: f64, t61322: f64, t69496: f64, t69505: f64, t69519: f64, t69529: f64, t69570: f64, t69579: f64, t76585: f64, t76608: f64, t76616: f64, t76624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76668, t76671, t76674, t76675, t76715) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1348(t21303, t49274, t10704, t42028, t76644, t21239, t4488, t959, t5950, t5919, t5943, t10165, t1052, t1634, t1635, t17588, t18074, t21662, t21663, t21677, t21692, t3174, t388, t43604, t4557, t4660, t5848, t5914, t5920, t69871, t70978, t70980);
        let (t76722, t76740, t76768) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1349(t5866, t5872, t1021, t10408, t1041, t10413, t10482, t1622, t17177, t17607, t17923, t18030, t21393, t21398, t21516, t248, t28651, t3039, t3070, t3071, t360, t43291, t43292, t43385, t43399, t4644, t48570, t50265, t5857, t5861, t5869, t5875, t61663, t61736, t70122, t70132, t70138, t70153, t76572);
        let t76829 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1350(t5836, t5842, t1539, t17800, t17817, t17863, t2986, t2994, t340, t343, t42861, t42862, t4531, t4546, t61365, t69487, t69503, t69515, t69540, t7577, t75836, t75847, t75912, t973, t974, t977, t978);
        let t76865 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1351(t13798, t17794, t17800, t17804, t17817, t17863, t2986, t42817, t4510, t4514, t4518, t4531, t48221, t61322, t69496, t69505, t69519, t69529, t69570, t69579, t76585, t76608, t76616, t76624);
    (t76668, t76671, t76674, t76675, t76715, t76722, t76740, t76768, t76829, t76865)
}
