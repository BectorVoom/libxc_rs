//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1348;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1349;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1350;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta461<F: Float>(t21303: F, t49274: F, t10704: F, t42028: F, t76644: F, t21239: F, t4488: F, t959: F, t5950: F, t5919: F, t5943: F, t10165: F, t1052: F, t1634: F, t1635: F, t17588: F, t18074: F, t21662: F, t21663: F, t21677: F, t21692: F, t3174: F, t388: F, t43604: F, t4557: F, t4660: F, t5848: F, t5914: F, t5920: F, t69871: F, t70978: F, t70980: F, t5866: F, t5872: F, t1021: F, t10408: F, t1041: F, t10413: F, t10482: F, t1622: F, t17177: F, t17607: F, t17923: F, t18030: F, t21393: F, t21398: F, t21516: F, t248: F, t28651: F, t3039: F, t3070: F, t3071: F, t360: F, t43291: F, t43292: F, t43385: F, t43399: F, t4644: F, t48570: F, t50265: F, t5857: F, t5861: F, t5869: F, t5875: F, t61663: F, t61736: F, t70122: F, t70132: F, t70138: F, t70153: F, t76572: F, t5836: F, t5842: F, t1539: F, t17800: F, t17817: F, t17863: F, t2986: F, t2994: F, t340: F, t343: F, t42861: F, t42862: F, t4531: F, t4546: F, t61365: F, t69487: F, t69503: F, t69515: F, t69540: F, t7577: F, t75836: F, t75847: F, t75912: F, t973: F, t974: F, t977: F, t978: F, t13798: F, t17794: F, t17804: F, t42817: F, t4510: F, t4514: F, t4518: F, t48221: F, t61322: F, t69496: F, t69505: F, t69519: F, t69529: F, t69570: F, t69579: F, t76585: F, t76608: F, t76616: F, t76624: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t76668, t76671, t76674, t76675, t76715) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1348::<F>(t21303, t49274, t10704, t42028, t76644, t21239, t4488, t959, t5950, t5919, t5943, t10165, t1052, t1634, t1635, t17588, t18074, t21662, t21663, t21677, t21692, t3174, t388, t43604, t4557, t4660, t5848, t5914, t5920, t69871, t70978, t70980);
        let (t76722, t76740, t76768) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1349::<F>(t5866, t5872, t1021, t10408, t1041, t10413, t10482, t1622, t17177, t17607, t17923, t18030, t21393, t21398, t21516, t248, t28651, t3039, t3070, t3071, t360, t43291, t43292, t43385, t43399, t4644, t48570, t50265, t5857, t5861, t5869, t5875, t61663, t61736, t70122, t70132, t70138, t70153, t76572);
        let t76829 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1350::<F>(t5836, t5842, t1539, t17800, t17817, t17863, t2986, t2994, t340, t343, t42861, t42862, t4531, t4546, t61365, t69487, t69503, t69515, t69540, t7577, t75836, t75847, t75912, t973, t974, t977, t978);
        let t76865 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1351::<F>(t13798, t17794, t17800, t17804, t17817, t17863, t2986, t42817, t4510, t4514, t4518, t4531, t48221, t61322, t69496, t69505, t69519, t69529, t69570, t69579, t76585, t76608, t76616, t76624);
    (t76668, t76671, t76674, t76675, t76715, t76722, t76740, t76768, t76829, t76865)
}
