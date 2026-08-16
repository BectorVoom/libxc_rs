//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta770 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2670;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta770<F: Float>(t53796: F, t53798: F, t39305: F, t1799: F, t3914: F, t12477: F, t20067: F, t3734: F, t3918: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t39316: F, t5126: F, t5161: F, t6330: F, t12461: F, t6463: F, t54312: F, t54314: F, t5356: F, t54316: F, t1307: F, t16018: F, t193: F, t19631: F, t20081: F, t3698: F, t3701: F, t3719: F, t39320: F, t39324: F, t39327: F, t5127: F, t5160: F, t533: F, t571: F) -> (F, F, F, F, F, F, F, F) {
        let (t56114, t56115, t56119, t56124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2670::<F>(t53796, t53798, t39305, t1799, t3914, t12477, t20067, t3734, t3918, t39261, t39266, t39304, t39309, t39312, t39316, t5126, t5161, t6330);
        let (t56140, t56141, t56147, t56148) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2671::<F>(t12461, t6463, t54312, t54314, t5356, t54316, t1307, t16018, t193, t19631, t20081, t3698, t3701, t3719, t39320, t39324, t39327, t5126, t5127, t5160, t533, t571);
    (t56114, t56115, t56119, t56124, t56140, t56141, t56147, t56148)
}
