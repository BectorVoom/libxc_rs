//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta439<F: Float>(t22690: F, t6638: F, t23171: F, t828: F, t852: F, t232: F, t6646: F, t1888: F, t10097: F, t206: F, t268: F, t6559: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23172, t23173, t23176, t23177, t23178, t23180, t23181, t23182, t23185) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1622::<F>(t22690, t6638, t23171, t828, t852, t232, t6646, t1888, t10097, t206, t268, t6559);
    (t23172, t23173, t23176, t23177, t23178, t23180, t23181, t23182, t23185)
}
