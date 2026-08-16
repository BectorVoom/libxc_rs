//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1108;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta265<F: Float>(t6553: F, t7488: F, t1880: F, t1496: F, t6581: F, t1484: F, t236: F, t1894: F, t6591: F, t1510: F, t815: F, t6605: F, t1499: F, t1898: F, t249: F, t1512: F, t6614: F, t1516: F, t6621: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7489, t7490, t7494, t7496) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1108::<F>(t6553, t7488, t1880, t1496, t6581, t1484, t236);
        let (t7497, t7498, t7500, t7501, t7503, t7504, t7506, t7508) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1109::<F>(t1894, t7496, t6591, t1510, t815, t6605, t1499, t1898, t249, t1512, t6614, t1516, t6621);
    (t7489, t7490, t7494, t7496, t7497, t7498, t7500, t7501, t7503, t7504, t7506, t7508)
}
