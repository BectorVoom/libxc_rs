//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta770 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2670;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta770(t53796: f64, t53798: f64, t39305: f64, t1799: f64, t3914: f64, t12477: f64, t20067: f64, t3734: f64, t3918: f64, t39261: f64, t39266: f64, t39304: f64, t39309: f64, t39312: f64, t39316: f64, t5126: f64, t5161: f64, t6330: f64, t12461: f64, t6463: f64, t54312: f64, t54314: f64, t5356: f64, t54316: f64, t1307: f64, t16018: f64, t193: f64, t19631: f64, t20081: f64, t3698: f64, t3701: f64, t3719: f64, t39320: f64, t39324: f64, t39327: f64, t5127: f64, t5160: f64, t533: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56114, t56115, t56119, t56124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2670(t53796, t53798, t39305, t1799, t3914, t12477, t20067, t3734, t3918, t39261, t39266, t39304, t39309, t39312, t39316, t5126, t5161, t6330);
        let (t56140, t56141, t56147, t56148) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2671(t12461, t6463, t54312, t54314, t5356, t54316, t1307, t16018, t193, t19631, t20081, t3698, t3701, t3719, t39320, t39324, t39327, t5126, t5127, t5160, t533, t571);
    (t56114, t56115, t56119, t56124, t56140, t56141, t56147, t56148)
}
