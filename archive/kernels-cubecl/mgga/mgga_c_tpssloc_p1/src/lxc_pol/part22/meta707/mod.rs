//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2297;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2298;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2299;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2300;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta707<F: Float>(t20148: F, t580: F, t20186: F, t576: F, t1395: F, t6483: F, t1404: F, t6470: F, t1858: F, t5363: F, t22430: F, t111: F, t20292: F, t12725: F, t19451: F, t19456: F, t20100: F, t20109: F, t20136: F, t20717: F, t2314: F, t4028: F, t4034: F, t4072: F, t4077: F, t5107: F, t5460: F, t5493: F, t5494: F, t6287: F, t652: F, t672: F, t7458: F, t46125: F, t45869: F, t45870: F, t25: F, t28: F, zeta_threshold: F, t40: F, t12862: F, t12865: F, t16549: F, t16558: F, t17635: F, t20217: F, t20234: F, t2433: F, t3966: F, t40632: F, t4080: F, t5398: F, t607: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t66967, t66976, t66987, t66989, t66991, t67000, t67001) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2297::<F>(t20148, t580, t20186, t576, t1395, t6483, t1404, t6470, t1858, t5363, t22430, t111, t20292);
        let t67030 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2298::<F>(t12725, t19451, t19456, t20100, t20109, t20136, t20717, t2314, t4028, t4034, t4072, t4077, t5107, t5460, t5493, t5494, t6287, t652, t67001, t672, t7458);
        let (t67044, t67059) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2299::<F>(t46125, t45869, t45870);
        let t67060 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2300::<F>(t25, t28, t67059, zeta_threshold);
        let t67064 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2301::<F>(t40, t12862, t12865, t16549, t16558, t17635, t20217, t20234, t2433, t3966, t40632, t4080, t5398, t607, t67060, t73, zeta_threshold);
    (t66967, t66976, t66987, t66989, t66991, t67000, t67001, t67030, t67044, t67059, t67060, t67064)
}
