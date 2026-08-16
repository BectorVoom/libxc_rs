//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta211<F: Float>(t1409: F, t2517: F, t707: F, t1484: F, t212: F, t9523: F, t2586: F, t2570: F, t67: F, t792: F, t131: F, t9558: F) -> (F, F, F, F, F, F, F) {
        let (t12945, t12946, t12984, t12985, t12986, t12998, t13004) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk854::<F>(t1409, t2517, t707, t1484, t212, t9523, t2586, t2570, t67, t792, t131, t9558);
    (t12945, t12946, t12984, t12985, t12986, t12998, t13004)
}
