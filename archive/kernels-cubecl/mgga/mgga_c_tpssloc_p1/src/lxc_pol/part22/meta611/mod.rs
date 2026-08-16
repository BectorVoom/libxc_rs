//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2139;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta611<F: Float>(t11274: F, t1657: F, t50826: F, t50853: F, t3263: F, t4737: F, t11189: F, t50919: F, t50948: F, t51039: F, t51051: F, t3400: F, t4832: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51120, t51137, t51151, t51246, t51249, t51257, t51271, t51299, t51310, t51349, t51354, t51371) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2139::<F>(t11274, t1657, t50826, t50853, t3263, t4737, t11189, t50919, t50948, t51039, t51051, t3400, t4832);
    (t51120, t51137, t51151, t51246, t51249, t51257, t51271, t51299, t51310, t51349, t51354, t51371)
}
