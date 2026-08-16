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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta651(t2018: f64, t3734: f64, t1983: f64, t7687: f64, t26062: f64, t645: f64, t72: f64, t26066: f64, t2307: f64, t7431: f64, t26012: f64, t6505: f64, t1437: f64, t6509: f64, t1863: f64, t1864: f64, t4021: f64, t1410: f64, t9231: f64, t2240: f64, t3961: f64, t3967: f64, t22544: f64, t22549: f64, t22551: f64, t26009: f64, t26013: f64, t83722: f64, t83741: f64, t83778: f64, t12571: f64, t608: f64, t33: f64, t46099: f64, t2244: f64, t3953: f64, t1865: f64, t22513: f64, t22516: f64, t22534: f64, t26016: f64, t26028: f64, t6506: f64, t6510: f64, t7428: f64, t7442: f64, t7446: f64, t83725: f64, t83729: f64, t83738: f64, t9239: f64, t2241: f64, t12648: f64, t605: f64, t12652: f64, t12661: f64, t26070: f64, t26073: f64, t26076: f64, t83719: f64, t83827: f64, t83830: f64, t4017: f64, t46104: f64, t6489: f64, t22522: f64, t26083: f64, t1433: f64, t22519: f64, t22523: f64, t22527: f64, t22531: f64, t22546: f64, t22554: f64, t26021: f64, t26025: f64, t26051: f64, t26090: f64, t6490: f64, t6492: f64, t6495: f64, t12568: f64, t2251: f64, t2303: f64, t26055: f64, t26063: f64, t26067: f64, t7432: f64, t83750: f64, t83760: f64, t83775: f64, t641: f64, t7445: f64, t22550: f64, t7441: f64, t12619: f64, t71: f64, t1860: f64, t22490: f64, t22493: f64, t22512: f64, t26024: f64, t31683: f64, t6486: f64, t26043: f64, t12606: f64, t1409: f64, t14165: f64, t22489: f64, t22502: f64, t22505: f64, t22537: f64, t26044: f64, t26045: f64, t26048: f64, t3966: f64, t6500: f64, t67: f64, t7435: f64, t83788: f64, t83791: f64, t83796: f64, t83803: f64, t45844: f64, t12719: f64, t79: f64, t9228: f64, t2235: f64, t26084: f64, t83814: f64, t5: f64, t112: f64, t2319: f64, t7450: f64, t26117: f64, t6534: f64) -> (f64, f64, f64, f64) {
        let (t90068, t90072, t90076, t90080, t90087) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2264(t2018, t3734, t1983, t7687, t26062, t645, t72, t26066, t2307, t7431, t26012, t6505);
        let t90107 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2265(t1437, t6509, t1863, t1864, t4021, t1410, t9231, t2240, t3961, t3967, t22544, t22549, t22551, t26009, t26013, t83722, t83741, t83778, t90072, t90076, t90080, t90087);
        let t90135 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2266(t12571, t608, t33, t46099, t2244, t3953, t1865, t22513, t22516, t22534, t22551, t26016, t26028, t6506, t6510, t7428, t7442, t7446, t83725, t83729, t83738);
        let t90167 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2267(t1410, t9239, t2241, t72, t7431, t12648, t605, t12652, t12661, t1865, t26009, t26070, t26073, t26076, t6506, t6510, t83719, t83827, t83830);
        let t90199 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2268(t4017, t645, t72, t46104, t6489, t12571, t22522, t26083, t9239, t1433, t2241, t22519, t22523, t22527, t22531, t22544, t22546, t22554, t26021, t26025, t26051, t26090, t6490, t6492, t6495, t7446);
        let t90230 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2269(t12568, t608, t2251, t3953, t1437, t2303, t72, t1865, t22523, t22554, t26055, t26063, t26067, t6490, t6506, t6510, t7432, t83750, t83760, t83775);
        let t90265 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2270(t4021, t641, t72, t645, t7445, t1863, t22550, t7441, t12619, t71, t1860, t22490, t22493, t22512, t22549, t26009, t26021, t26024, t26025, t31683, t6486, t6490, t6505, t7428, t7442, t7446, t9239);
        let t90315 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2271(t1433, t2307, t72, t26083, t9231, t2240, t26043, t33, t12606, t12648, t12652, t1409, t14165, t1860, t1864, t22489, t22490, t22502, t22505, t22513, t22516, t22537, t26044, t26045, t26048, t3961, t3966, t6486, t6490, t6492, t6500, t6509, t67, t7435, t7441, t7446, t83788, t83791, t83796, t83803);
        let t90346 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2272(t45844, t6489, t12719, t72, t79, t1410, t9228, t2235, t3961, t3967, t1865, t22519, t22527, t22531, t22537, t22546, t26045, t26048, t26084, t6490, t6495, t7432, t7442, t83814);
        let (t90351, t90352, t90355) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2273(t5, t90107, t90135, t90167, t90199, t90230, t90265, t90315, t90346, t112, t2319, t7450, t26117, t6534);
    (t90068, t90351, t90352, t90355)
}
