//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1486;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta371<F: Float>(t12907: F, t13475: F, t13483: F, t13491: F, t2: F, t873: F, t584: F, t265: F, t16: F, t4331: F, t10723: F, t4496: F, t959: F, t2944: F, t4483: F, t2940: F, t4493: F, t4351: F, t892: F, t914: F, t2837: F, t4354: F, t1543: F, t2841: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13493, t13503, t13504, t13506, t13508) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1486::<F>(t12907, t13475, t13483, t13491, t2, t873, t584, t265, t16, t4331, t10723, t4496);
        let (t13510, t13512, t13514, t13517, t13519, t13520) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1487::<F>(t13508, t959, t2944, t4483, t2940, t4493, t4351, t892, t914, t2837, t4354, t1543, t2841);
    (t13493, t13503, t13504, t13506, t13510, t13512, t13514, t13517, t13519, t13520)
}
