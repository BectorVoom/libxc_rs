//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1638;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta477<F: Float>(t26309: F, t5252: F, t22833: F, t5293: F, t5303: F, t1351: F, t16311: F, t3788: F, t6936: F, t16306: F, t550: F, t1339: F, t1887: F, t22839: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26310, t26312, t26314, t26318, t26319, t26320, t26322, t26323) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1638::<F>(t26309, t5252, t22833, t5293, t5303, t1351, t16311, t3788, t6936, t16306, t550, t1339);
        let (t26324, t26331) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1639::<F>(t26323, t6936, t1887, t22839);
    (t26310, t26312, t26314, t26318, t26319, t26320, t26322, t26323, t26324, t26331)
}
