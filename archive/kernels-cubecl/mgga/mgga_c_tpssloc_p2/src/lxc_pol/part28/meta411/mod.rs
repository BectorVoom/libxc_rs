//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta411<F: Float>(t3791: F, t562: F, t550: F, t6976: F, t1992: F, t6914: F, t6979: F, t3734: F, t6968: F, t6637: F, t22685: F, t6546: F, t6887: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22740, t22741, t22742, t22743, t22745, t22746, t22747, t22748, t22749, t22751) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1580::<F>(t3791, t562, t550, t6976, t1992, t6914, t6979, t3734, t6968, t6637, t22685, t6546, t6887);
    (t22740, t22741, t22742, t22743, t22745, t22746, t22747, t22748, t22749, t22751)
}
