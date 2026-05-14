//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 914/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk914<F: Float>(t144: F, t26527: F, t558: F, t574: F, t6725: F, t1391: F, t3408: F, t1882: F, t6701: F, t26777: F, t6720: F, t23534: F, t23546: F, t23576: F, t23598: F, t23943: F, t23945: F, t23947: F, t23950: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t27289 = t144 * t26527;
    let t27295 = t574 * t6725 * t558;
    let t27299 = t574 * t1391 * t3408;
    let t27302 = t1882 * t6701;
    let t27307 = t144 * t26777;
    let t27310 = t1882 * t6720;
    let t27312 = -t23534 / 9.0 + t23546 / 9.0 + 2.0 / 3.0 * t446 * t27289 - t23576 / 27.0 + t23598 / 27.0 - t446 * t27295 / 3.0 - t446 * t27299 / 3.0 - t27302 / 9.0 + t23943 / 9.0 + t23945 / 9.0 + t23947 / 9.0 - t446 * t27307 / 3.0 - t23950 + t27310 / 9.0;
    (t27289, t27295, t27299, t27302, t27307, t27310, t27312)
}
