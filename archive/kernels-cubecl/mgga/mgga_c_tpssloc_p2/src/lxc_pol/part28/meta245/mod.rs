//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1069;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta245<F: Float>(t252: F, t828: F, t232: F, t6646: F, t1888: F, t1894: F, t852: F, t214: F, t1880: F, t25: F, t868: F, t343: F, t984: F, t3034: F, t334: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t6648, t6649, t6650, t6652, t6653, t6654, t6671, t6733) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1069::<F>(t252, t828, t232, t6646, t1888, t1894, t852, t214, t1880, t25, t868, t343, t984);
        let t6739 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1070::<F>(t3034, t334);
    (t6648, t6649, t6650, t6652, t6653, t6654, t6671, t6733, t6739)
}
