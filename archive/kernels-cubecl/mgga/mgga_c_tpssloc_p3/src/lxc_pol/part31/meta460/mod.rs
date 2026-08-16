//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1612;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1613;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta460<F: Float>(t23185: F, t25245: F, t234: F, t6604: F, t1484: F, t252: F, t776: F, t25038: F, t7528: F, t794: F, t6562: F, t13380: F, t232: F, t6646: F, t1888: F, t6579: F, t7525: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25246, t25248) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1612::<F>(t23185, t25245, t234, t6604);
        let t25249 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1613::<F>(t1484, t252);
        let (t25250, t25251, t25252, t25258, t25259, t25272, t25273, t25274, t25277) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1614::<F>(t25249, t776, t25248, t25038, t7528, t794, t6562, t13380, t232, t6646, t1888, t6579, t7525);
    (t25246, t25248, t25249, t25250, t25251, t25252, t25258, t25259, t25272, t25273, t25274, t25277)
}
