//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1081/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1081<F: Float>(t2413: F, t6852: F, t446: F, t9770: F, t17864: F, t24265: F, t697: F, t22511: F, t27519: F, t3789: F, t27617: F, t2917: F, t24372: F, t27561: F, t13475: F, t2418: F) -> (F, F, F, F, F, F, F) {
    let t108439 = t6852 * t2413;
    let t108441 = t446 * t9770 * t108439;
    let t108445 = 0.29693535778629056444e-3 * t24265 * t697 * t17864;
    let t108446 = t27519 * t22511;
    let t108447 = t3789 * t108446;
    let t108448 = t27617 * t2917;
    let t108454 = t24372 * t697 * t27561;
    let t108456 = t13475 * t2418;
    (t108439, t108441, t108445, t108447, t108448, t108454, t108456)
}
