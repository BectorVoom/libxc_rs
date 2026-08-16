//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta687<F: Float>(t19451: F, t6534: F, t1458: F, t4025: F, t1873: F, t55943: F, t19456: F, t7467: F, t26135: F, t4028: F, t5493: F, t649: F) -> (F, F, F, F, F, F, F) {
        let (t96681, t96683, t96685, t96704, t96706, t96708, t96709) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2129::<F>(t19451, t6534, t1458, t4025, t1873, t55943, t19456, t7467, t26135, t4028, t5493, t649);
    (t96681, t96683, t96685, t96704, t96706, t96708, t96709)
}
