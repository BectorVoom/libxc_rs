//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 712/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk712(t144: f64, t26527: f64, t558: f64, t574: f64, t6725: f64, t1391: f64, t3408: f64, t1882: f64, t6701: f64, t26777: f64, t6720: f64, t23534: f64, t23546: f64, t23576: f64, t23598: f64, t23943: f64, t23945: f64, t23947: f64, t23950: f64, t446: f64) -> f64 {
    let t27289 = t144 * t26527;
    let t27295 = t574 * t6725 * t558;
    let t27299 = t574 * t1391 * t3408;
    let t27302 = t1882 * t6701;
    let t27307 = t144 * t26777;
    let t27310 = t1882 * t6720;
    let t27312 = -t23534 / 9.0_f64 + t23546 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t27289 - t23576 / 27.0_f64 + t23598 / 27.0_f64 - t446 * t27295 / 3.0_f64 - t446 * t27299 / 3.0_f64 - t27302 / 9.0_f64 + t23943 / 9.0_f64 + t23945 / 9.0_f64 + t23947 / 9.0_f64 - t446 * t27307 / 3.0_f64 - t23950 + t27310 / 9.0_f64;
    t27312
}
