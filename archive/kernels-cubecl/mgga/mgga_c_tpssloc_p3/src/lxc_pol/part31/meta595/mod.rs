//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta595<F: Float>(t12571: F, t23966: F, t6492: F, t7432: F, t84195: F, t23967: F, t26067: F, t23993: F, t7428: F, t23998: F, t1860: F, t23992: F, t7445: F) -> (F, F, F, F, F, F, F) {
        let (t91957, t91959, t91961, t91980, t91996, t92001, t92003) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1840::<F>(t12571, t23966, t6492, t7432, t84195, t23967, t26067, t23993, t7428, t23998, t1860, t23992, t7445);
    (t91957, t91959, t91961, t91980, t91996, t92001, t92003)
}
