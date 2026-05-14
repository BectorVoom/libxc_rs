//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1181/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1181<F: Float>(t1882: F, t29295: F, t7021: F, t870: F, t29158: F, t29371: F, t29087: F, t8392: F, t29134: F, t29401: F, t29309: F, t29350: F, t29250: F, t29077: F, t29278: F, t29299: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t114569 = 2.0 / 9.0 * t1882 * t29295;
    let t114578 = t870 * t7021;
    let t114595 = 2.0 / 9.0 * t1882 * t29158;
    let t114606 = 2.0 / 9.0 * t1882 * t29371;
    let t114616 = 2.0 / 27.0 * t8392 * t29087;
    let t114626 = 2.0 / 27.0 * t8392 * t29134;
    let t114648 = 2.0 / 9.0 * t1882 * t29401;
    let t114683 = 2.0 / 9.0 * t1882 * t29309;
    let t114694 = 4.0 / 9.0 * t1882 * t29350;
    let t114707 = 4.0 / 9.0 * t1882 * t29250;
    let t114726 = 4.0 / 9.0 * t8392 * t29077;
    let t114728 = 2.0 / 9.0 * t1882 * t29278;
    let t114734 = 2.0 / 9.0 * t1882 * t29299;
    (t114569, t114578, t114595, t114606, t114616, t114626, t114648, t114683, t114694, t114707, t114726, t114728, t114734)
}
