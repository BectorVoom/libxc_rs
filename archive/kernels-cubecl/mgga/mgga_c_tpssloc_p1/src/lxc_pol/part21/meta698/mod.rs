//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta698 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2525;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta698<F: Float>(t4542: F, t698: F, t973: F, t10186: F, t13788: F, t13560: F, t699: F, t2403: F, t4392: F, t13646: F, t1553: F, t9709: F, t13538: F, t133: F, t135: F, t241: F) -> (F, F, F, F, F, F, F, F) {
        let (t48066, t48068, t48087, t48096, t48098, t48103) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2525::<F>(t4542, t698, t973, t10186, t13788, t13560, t699, t2403, t4392, t13646, t1553, t9709);
        let (t48116, t48140) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2526::<F>(t13538, t699, t133, t135, t241);
    (t48066, t48068, t48087, t48096, t48098, t48103, t48116, t48140)
}
