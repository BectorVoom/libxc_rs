//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta640<F: Float>(t22960: F, t98007: F, t5660: F, t776: F, t67164: F, t1408: F, t4119: F, t1530: F, t4303: F, t25373: F, t67123: F, t5544: F, t606: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98008, t98011, t98012, t98015, t98020, t98030, t98031, t98034, t98046) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1908::<F>(t22960, t98007, t5660, t776, t67164, t1408, t4119, t1530, t4303, t25373, t67123, t5544, t606);
    (t98008, t98011, t98012, t98015, t98020, t98030, t98031, t98034, t98046)
}
