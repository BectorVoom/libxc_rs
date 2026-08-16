//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta701<F: Float>(t10186: F, t13785: F, t13839: F, t2986: F, t42837: F, t10236: F, t12652: F, t12648: F, t13783: F, t1597: F, t10237: F, t340: F, t4548: F, t698: F, t973: F) -> (F, F, F, F, F, F, F) {
        let (t48244, t48250, t48256, t48269, t48279, t48281, t48292) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2530::<F>(t10186, t13785, t13839, t2986, t42837, t10236, t12652, t12648, t13783, t1597, t10237, t340, t4548, t698, t973);
    (t48244, t48250, t48256, t48269, t48279, t48281, t48292)
}
