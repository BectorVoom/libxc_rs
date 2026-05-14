//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 849/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk849<F: Float>(t27468: F, t505: F, t2354: F, t446: F, t1882: F, t6896: F, t18: F, t6135: F, t3281: F, t10157: F, t6852: F, t713: F, t24526: F, t992: F, t1411: F, t3758: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27469 = t27468 * t505;
    let t27470 = t2354 * t27469;
    let t27471 = t446 * t27470;
    let t27473 = t1882 * t6896;
    let t27475 = t6135 * t18;
    let t27476 = t2354 * t27475;
    let t27477 = t3281 * t27476;
    let t27480 = t10157 * t6852 * t713;
    let t27481 = t446 * t27480;
    let t27483 = t24526 * t992;
    let t27484 = t2354 * t27483;
    let t27485 = t446 * t27484;
    let t27487 = t3758 * t1411;
    (t27470, t27471, t27473, t27476, t27477, t27480, t27481, t27484, t27485, t27487)
}
