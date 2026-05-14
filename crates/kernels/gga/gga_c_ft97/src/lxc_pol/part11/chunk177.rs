//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 177/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk177<F: Float>(t79: F, t409: F, t428: F, t372: F, t385: F, t399: F, t403: F, t64: F) -> (F, F) {
    let t80 = 0.1e-59 < t79;
    let t429 = t409 * t428;
    let t432 = piecewise3(t80, -0.11627450473218896e-1 * t372 * t385 + 2.0 * t403 + 0.59273806478425129876e-2 * t79 * t399 - t64 * t429, 0.0);
    (t429, t432)
}
