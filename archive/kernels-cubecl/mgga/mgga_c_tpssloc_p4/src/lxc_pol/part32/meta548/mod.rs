//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta548<F: Float>(t460: F, t491: F, t7286: F, t27453: F, t27721: F, t466: F, t7280: F, t7999: F, t1186: F, t8010: F, t1170: F, t2121: F) -> (F, F, F, F, F, F, F, F) {
        let (t27798, t27799, t27800, t27805, t27808, t27812, t27817, t27818) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1899::<F>(t460, t491, t7286, t27453, t27721, t466, t7280, t7999, t1186, t8010, t1170, t2121);
    (t27798, t27799, t27800, t27805, t27808, t27812, t27817, t27818)
}
