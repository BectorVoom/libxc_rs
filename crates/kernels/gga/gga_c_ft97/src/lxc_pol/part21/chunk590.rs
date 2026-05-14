//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 590/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk590<F: Float>(t1780: F, t488: F, t3172: F, t376: F, t89: F, t1637: F, t973: F, t480: F, t2999: F, t443: F, t444: F) -> (F, F, F, F, F) {
    let t11556 = t1780 * t488;
    let t11567 = 2.0 / 9.0 * t89 * t376 * t3172;
    let t11578 = t89 * t1637 * t973;
    let t11587 = t1780 * t480;
    let t11593 = t443 * t444 * t2999;
    (t11556, t11567, t11578, t11587, t11593)
}
