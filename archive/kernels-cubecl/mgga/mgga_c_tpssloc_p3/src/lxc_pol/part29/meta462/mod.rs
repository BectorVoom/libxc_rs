//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1787;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta462<F: Float>(t23092: F, t23149: F, t235: F, t234: F, t852: F, t776: F, t6637: F, t6552: F, t2553: F, t6638: F, t117: F, t229: F, t67: F, t6559: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23150, t23151, t23153, t23154, t23155, t23156, t23158, t23159, t23160, t23163) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1787::<F>(t23092, t23149, t235, t234, t852, t776, t6637, t6552, t2553, t6638, t117, t229, t67);
        let t23164 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1788::<F>(t23163, t6559);
    (t23150, t23151, t23153, t23154, t23155, t23156, t23158, t23159, t23160, t23163, t23164)
}
