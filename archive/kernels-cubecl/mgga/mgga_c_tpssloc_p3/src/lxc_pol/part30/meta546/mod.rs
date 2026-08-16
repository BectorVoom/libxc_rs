//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta546<F: Float>(t27948: F, t67: F, t1864: F, t7441: F, t7445: F, t5441: F, t71: F, t1863: F, t5389: F, t79: F, t72: F, t1410: F, t3953: F) -> (F, F, F, F, F, F, F, F) {
        let (t27949, t27950, t27953, t27956, t27957, t27960, t27961, t27966) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1897::<F>(t27948, t67, t1864, t7441, t7445, t5441, t71, t1863, t5389, t79, t72, t1410, t3953);
    (t27949, t27950, t27953, t27956, t27957, t27960, t27961, t27966)
}
