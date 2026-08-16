//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta529<F: Float>(t5584: F, t852: F, t1509: F, t4265: F, t1519: F, t4233: F, t16752: F, t252: F, t5527: F, t828: F, t5611: F, t9975: F) -> (F, F, F, F, F, F, F, F) {
        let (t58166, t58204, t58226, t58262, t58557, t58569, t58688, t58853) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1743::<F>(t5584, t852, t1509, t4265, t1519, t4233, t16752, t252, t5527, t828, t5611, t9975);
    (t58166, t58204, t58226, t58262, t58557, t58569, t58688, t58853)
}
