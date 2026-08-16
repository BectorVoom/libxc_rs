//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta552<F: Float>(t111: F, t7222: F, t81437: F, t22550: F, t7031: F, t39054: F, t7025: F, t23966: F, t9231: F, t39063: F, t9239: F, t1860: F, t23992: F, t6509: F) -> (F, F, F, F, F, F, F, F) {
        let (t84033, t84036, t84173, t84190, t84195, t84216, t84219, t84229) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1780::<F>(t111, t7222, t81437, t22550, t7031, t39054, t7025, t23966, t9231, t39063, t9239, t1860, t23992, t6509);
    (t84033, t84036, t84173, t84190, t84195, t84216, t84219, t84229)
}
