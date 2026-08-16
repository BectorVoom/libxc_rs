//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta651 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2264;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2265;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2266;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2267;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2268;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2269;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2270;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2271;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2272;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta651<F: Float>(t2018: F, t3734: F, t1983: F, t7687: F, t26062: F, t645: F, t72: F, t26066: F, t2307: F, t7431: F, t26012: F, t6505: F, t1437: F, t6509: F, t1863: F, t1864: F, t4021: F, t1410: F, t9231: F, t2240: F, t3961: F, t3967: F, t22544: F, t22549: F, t22551: F, t26009: F, t26013: F, t83722: F, t83741: F, t83778: F, t12571: F, t608: F, t33: F, t46099: F, t2244: F, t3953: F, t1865: F, t22513: F, t22516: F, t22534: F, t26016: F, t26028: F, t6506: F, t6510: F, t7428: F, t7442: F, t7446: F, t83725: F, t83729: F, t83738: F, t9239: F, t2241: F, t12648: F, t605: F, t12652: F, t12661: F, t26070: F, t26073: F, t26076: F, t83719: F, t83827: F, t83830: F, t4017: F, t46104: F, t6489: F, t22522: F, t26083: F, t1433: F, t22519: F, t22523: F, t22527: F, t22531: F, t22546: F, t22554: F, t26021: F, t26025: F, t26051: F, t26090: F, t6490: F, t6492: F, t6495: F, t12568: F, t2251: F, t2303: F, t26055: F, t26063: F, t26067: F, t7432: F, t83750: F, t83760: F, t83775: F, t641: F, t7445: F, t22550: F, t7441: F, t12619: F, t71: F, t1860: F, t22490: F, t22493: F, t22512: F, t26024: F, t31683: F, t6486: F, t26043: F, t12606: F, t1409: F, t14165: F, t22489: F, t22502: F, t22505: F, t22537: F, t26044: F, t26045: F, t26048: F, t3966: F, t6500: F, t67: F, t7435: F, t83788: F, t83791: F, t83796: F, t83803: F, t45844: F, t12719: F, t79: F, t9228: F, t2235: F, t26084: F, t83814: F, t5: F, t112: F, t2319: F, t7450: F, t26117: F, t6534: F) -> (F, F, F, F) {
        let (t90068, t90072, t90076, t90080, t90087) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2264::<F>(t2018, t3734, t1983, t7687, t26062, t645, t72, t26066, t2307, t7431, t26012, t6505);
        let t90107 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2265::<F>(t1437, t6509, t1863, t1864, t4021, t1410, t9231, t2240, t3961, t3967, t22544, t22549, t22551, t26009, t26013, t83722, t83741, t83778, t90072, t90076, t90080, t90087);
        let t90135 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2266::<F>(t12571, t608, t33, t46099, t2244, t3953, t1865, t22513, t22516, t22534, t22551, t26016, t26028, t6506, t6510, t7428, t7442, t7446, t83725, t83729, t83738);
        let t90167 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2267::<F>(t1410, t9239, t2241, t72, t7431, t12648, t605, t12652, t12661, t1865, t26009, t26070, t26073, t26076, t6506, t6510, t83719, t83827, t83830);
        let t90199 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2268::<F>(t4017, t645, t72, t46104, t6489, t12571, t22522, t26083, t9239, t1433, t2241, t22519, t22523, t22527, t22531, t22544, t22546, t22554, t26021, t26025, t26051, t26090, t6490, t6492, t6495, t7446);
        let t90230 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2269::<F>(t12568, t608, t2251, t3953, t1437, t2303, t72, t1865, t22523, t22554, t26055, t26063, t26067, t6490, t6506, t6510, t7432, t83750, t83760, t83775);
        let t90265 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2270::<F>(t4021, t641, t72, t645, t7445, t1863, t22550, t7441, t12619, t71, t1860, t22490, t22493, t22512, t22549, t26009, t26021, t26024, t26025, t31683, t6486, t6490, t6505, t7428, t7442, t7446, t9239);
        let t90315 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2271::<F>(t1433, t2307, t72, t26083, t9231, t2240, t26043, t33, t12606, t12648, t12652, t1409, t14165, t1860, t1864, t22489, t22490, t22502, t22505, t22513, t22516, t22537, t26044, t26045, t26048, t3961, t3966, t6486, t6490, t6492, t6500, t6509, t67, t7435, t7441, t7446, t83788, t83791, t83796, t83803);
        let t90346 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2272::<F>(t45844, t6489, t12719, t72, t79, t1410, t9228, t2235, t3961, t3967, t1865, t22519, t22527, t22531, t22537, t22546, t26045, t26048, t26084, t6490, t6495, t7432, t7442, t83814);
        let (t90351, t90352, t90355) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2273::<F>(t5, t90107, t90135, t90167, t90199, t90230, t90265, t90315, t90346, t112, t2319, t7450, t26117, t6534);
    (t90068, t90351, t90352, t90355)
}
