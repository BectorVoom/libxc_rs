//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta429<F: Float>(t23041: F, t831: F, t2686: F, t6614: F, t2627: F, t59: F, t240: F, t812: F, t2635: F, t2681: F, t2617: F, t6613: F) -> (F, F, F, F, F, F, F, F) {
        let (t23042, t23043, t23044, t23046, t23047, t23049, t23051, t23053) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1606::<F>(t23041, t831, t2686, t6614, t2627, t59, t240, t812, t2635, t2681, t2617, t6613);
    (t23042, t23043, t23044, t23046, t23047, t23049, t23051, t23053)
}
