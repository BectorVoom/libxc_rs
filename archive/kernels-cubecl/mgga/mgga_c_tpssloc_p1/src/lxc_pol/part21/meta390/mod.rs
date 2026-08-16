//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta390<F: Float>(t10277: F, t3061: F, t14165: F, t4582: F, t12652: F, t4588: F, t12648: F, t10216: F, t10969: F, t135: F, t4608: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14172, t14173, t14174, t14179, t14180, t14183, t14184, t14187, t14188, t14189, t14192, t14194) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1859::<F>(t10277, t3061, t14165, t4582, t12652, t4588, t12648, t10216, t10969, t135, t4608, t973);
    (t14172, t14173, t14174, t14179, t14180, t14183, t14184, t14187, t14188, t14189, t14192, t14194)
}
