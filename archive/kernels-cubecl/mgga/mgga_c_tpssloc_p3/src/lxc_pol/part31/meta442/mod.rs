//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta442<F: Float>(t23967: F, t6492: F, t2031: F, t22550: F, t6495: F, t7032: F, t7025: F, t9231: F, t6486: F, t240: F, t67: F, t1864: F) -> (F, F, F, F, F, F, F) {
        let (t23968, t23970, t23973, t23975, t23978, t23992, t23993) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1589::<F>(t23967, t6492, t2031, t22550, t6495, t7032, t7025, t9231, t6486, t240, t67, t1864);
    (t23968, t23970, t23973, t23975, t23978, t23992, t23993)
}
