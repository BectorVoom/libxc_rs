//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1120/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1120<F: Float>(t108431: F, t1882: F, t27869: F, t17864: F, t24265: F, t697: F, t22511: F, t27519: F, t3789: F, t27617: F, t2917: F) -> (F, F, F, F, F, F, F) {
    let t108432 = 4.0 / 9.0 * t108431;
    let t108433 = t1882 * t27869;
    let t108434 = 4.0 / 27.0 * t108433;
    let t108445 = 0.29693535778629056444e-3 * t24265 * t697 * t17864;
    let t108446 = t27519 * t22511;
    let t108447 = t3789 * t108446;
    let t108448 = t27617 * t2917;
    (t108432, t108433, t108434, t108445, t108446, t108447, t108448)
}
