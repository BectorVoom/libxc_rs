//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta697 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2661;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2662;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2663;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta697(t1788: f64, t9212: f64, t9214: f64, t2223: f64, t5168: f64, t39328: f64, t39330: f64, t39334: f64, t39339: f64, t39341: f64, t15977: f64, t588: f64, t25: f64, t5157: f64, t9874: f64, t5137: f64, t591: f64, t11988: f64, t12061: f64, t1408: f64, t15937: f64, t15940: f64, t16: f64, t2: f64, t3664: f64, t39419: f64, t5134: f64, t514: f64, t53805: f64, t53808: f64, t53814: f64, t53817: f64, t584: f64, t606: f64, t9257: f64, zeta_threshold: f64, t28: f64, t5145: f64, t1081: f64, t11122: f64, t12001: f64, t12072: f64, t15952: f64, t15955: f64, t1649: f64, t3672: f64, t39436: f64, t5142: f64, t517: f64, t53832: f64, t53835: f64, t53841: f64, t53844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54313, t54315, t54317, t54318, t54319, t54320, t54321, t54322, t54323) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2661(t1788, t9212, t9214, t2223, t5168, t39328, t39330, t39334, t39339, t39341, t15977, t588);
        let (t54324, t54326, t54349) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2662(t25, t54323, t5157, t9874, t5137, t591, t11988, t12061, t1408, t15937, t15940, t16, t2, t3664, t39419, t5134, t514, t53805, t53808, t53814, t53817, t584, t606, t9257, zeta_threshold);
        let t54372 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2663(t28, t5145, t591, t1081, t11122, t12001, t12072, t15952, t15955, t16, t1649, t2, t3672, t39436, t5142, t517, t53832, t53835, t53841, t53844, t584, zeta_threshold);
    (t54313, t54315, t54317, t54318, t54319, t54320, t54321, t54322, t54324, t54326, t54349, t54372)
}
