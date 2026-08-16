//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta433<F: Float>(t23030: F, t6643: F, t131: F, t244: F, t209: F, t1878: F, t6612: F, t835: F, t812: F) -> (F, F, F, F, F, F) {
        let (t23031, t23033, t23034, t23035, t23040, t23041) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1567::<F>(t23030, t6643, t131, t244, t209, t1878, t6612, t835, t812);
    (t23031, t23033, t23034, t23035, t23040, t23041)
}
