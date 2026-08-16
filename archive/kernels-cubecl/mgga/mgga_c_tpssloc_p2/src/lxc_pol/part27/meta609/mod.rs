//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta609<F: Float>(t10870: F, t6765: F, t10489: F, t23436: F, t3113: F, t1036: F, t23465: F, t3082: F, t6759: F, t344: F, t607: F, t1009: F, t6740: F) -> (F, F, F, F, F, F, F) {
        let (t82875, t82877, t82880, t82883, t82885, t82890, t82892) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2082::<F>(t10870, t6765, t10489, t23436, t3113, t1036, t23465, t3082, t6759, t344, t607, t1009, t6740);
    (t82875, t82877, t82880, t82883, t82885, t82890, t82892)
}
