//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta445<F: Float>(t23095: F, t23105: F, t23107: F, t23140: F, t23143: F, t23013: F, t23031: F, t2047: F, t2627: F, t23173: F, t7084: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24218, t24220, t24221, t24230, t24231, t24246, t24250, t24255, t24265, t24269) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1593::<F>(t23095, t23105, t23107, t23140, t23143, t23013, t23031, t2047, t2627, t23173, t7084, t814);
    (t24218, t24220, t24221, t24230, t24231, t24246, t24250, t24255, t24265, t24269)
}
