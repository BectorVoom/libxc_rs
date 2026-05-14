//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1333/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1333<F: Float>(t107547: F, t107552: F, t107563: F, t107566: F, t107573: F, t107574: F, t119299: F, t119768: F, t12703: F, t13140: F, t1359: F, t144: F, t16977: F, t17355: F, t1901: F, t27015: F, t27034: F, t27096: F, t3429: F, t446: F, t574: F, t605: F, t63052: F, t9144: F, t96222: F, t96224: F, t96227: F) -> (F,) {
    let t121570 = t446 * t574 * t605 * t1359 * t17355 / 3.0 + 4.0 / 3.0 * t446 * t144 * t119299 - 8.0 / 27.0 * t107547 - 4.0 / 27.0 * t96222 - 4.0 / 27.0 * t96224 + t107552 - 4.0 / 9.0 * t1901 * t12703 * t119768 - 2.0 / 9.0 * t1901 * t9144 * t27034 * t3429 - 4.0 / 9.0 * t1901 * t63052 * t27096 - 2.0 / 3.0 * t1901 * t13140 * t27015 * t16977 + 4.0 / 27.0 * t96227 + 8.0 / 27.0 * t107563 - t107566 + t107573 - 8.0 / 27.0 * t107574;
    (t121570,)
}
