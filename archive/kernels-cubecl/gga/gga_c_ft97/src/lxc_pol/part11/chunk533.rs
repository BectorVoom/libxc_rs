//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 533/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk533<F: Float>(t408: F, t6: F, t1693: F, t1710: F, t139: F, t1995: F, t527: F, t135: F, t542: F, t1711: F, t39: F, t64: F, rho0: F) -> (F, F, F, F, F, F, F) {
    let t5566 = t408 * t6;
    let t5588 = t1693 * rho0;
    let t5596 = t1710 * t6;
    let t5784 = t139 * t6;
    let t5785 = t1995 * t5784;
    let t5802 = t527 * t5784;
    let t5818 = t542 * t135;
    let t7201 = t1711 * t39;
    let t7202 = t64 * t7201;
    (t5566, t5588, t5596, t5785, t5802, t5818, t7202)
}
