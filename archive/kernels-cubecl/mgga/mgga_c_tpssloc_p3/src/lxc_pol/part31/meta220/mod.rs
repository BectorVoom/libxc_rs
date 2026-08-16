//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk953;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta220<F: Float>(t381: F, t5848: F, t1603: F, t1625: F, t1044: F, t248: F, t5685: F, t3062: F, t5677: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5798: F, t5800: F, t5802: F, t5806: F, t5810: F, t5814: F, t360: F, t1021: F, t1615: F) -> (F, F, F, F, F, F, F, F) {
        let (t5849, t5851, t5857, t5861, t5866) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk953::<F>(t381, t5848, t1603, t1625, t1044, t248, t5685, t3062, t5677, t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814);
        let (t5867, t5869, t5872) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk954::<F>(t360, t5866, t1021, t248, t1615);
    (t5849, t5851, t5857, t5861, t5866, t5867, t5869, t5872)
}
