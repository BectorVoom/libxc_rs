//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2297;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2298;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2299;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2300;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta707(t20148: f64, t580: f64, t20186: f64, t576: f64, t1395: f64, t6483: f64, t1404: f64, t6470: f64, t1858: f64, t5363: f64, t22430: f64, t111: f64, t20292: f64, t12725: f64, t19451: f64, t19456: f64, t20100: f64, t20109: f64, t20136: f64, t20717: f64, t2314: f64, t4028: f64, t4034: f64, t4072: f64, t4077: f64, t5107: f64, t5460: f64, t5493: f64, t5494: f64, t6287: f64, t652: f64, t672: f64, t7458: f64, t46125: f64, t45869: f64, t45870: f64, t25: f64, t28: f64, zeta_threshold: f64, t40: f64, t12862: f64, t12865: f64, t16549: f64, t16558: f64, t17635: f64, t20217: f64, t20234: f64, t2433: f64, t3966: f64, t40632: f64, t4080: f64, t5398: f64, t607: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66967, t66976, t66987, t66989, t66991, t67000, t67001) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2297(t20148, t580, t20186, t576, t1395, t6483, t1404, t6470, t1858, t5363, t22430, t111, t20292);
        let t67030 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2298(t12725, t19451, t19456, t20100, t20109, t20136, t20717, t2314, t4028, t4034, t4072, t4077, t5107, t5460, t5493, t5494, t6287, t652, t67001, t672, t7458);
        let (t67044, t67059) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2299(t46125, t45869, t45870);
        let t67060 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2300(t25, t28, t67059, zeta_threshold);
        let t67064 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2301(t40, t12862, t12865, t16549, t16558, t17635, t20217, t20234, t2433, t3966, t40632, t4080, t5398, t607, t67060, t73, zeta_threshold);
    (t66967, t66976, t66987, t66989, t66991, t67000, t67001, t67030, t67044, t67059, t67060, t67064)
}
