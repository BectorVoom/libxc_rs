//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta568<F: Float>(t25064: F, t81788: F, t25135: F, t838: F, t2693: F, t7503: F, t25132: F, t81882: F, t6604: F, t81968: F, t23083: F, t25123: F) -> (F, F, F, F, F, F) {
        let (t87387, t87401, t87403, t87405, t87407, t87411) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1800::<F>(t25064, t81788, t25135, t838, t2693, t7503, t25132, t81882, t6604, t81968, t23083, t25123);
    (t87387, t87401, t87403, t87405, t87407, t87411)
}
