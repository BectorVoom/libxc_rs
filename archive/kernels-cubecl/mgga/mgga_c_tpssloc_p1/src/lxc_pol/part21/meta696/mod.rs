//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta696 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta696<F: Float>(t14165: F, t43070: F, t10190: F, t13835: F, t2986: F, t42841: F, t10254: F, t12652: F, t1597: F, t43052: F, t2990: F, t10255: F, t13847: F) -> (F, F, F, F, F, F, F) {
        let (t47927, t47938, t47941, t47966, t48019, t48021, t48024) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2523::<F>(t14165, t43070, t10190, t13835, t2986, t42841, t10254, t12652, t1597, t43052, t2990, t10255, t13847);
    (t47927, t47938, t47941, t47966, t48019, t48021, t48024)
}
