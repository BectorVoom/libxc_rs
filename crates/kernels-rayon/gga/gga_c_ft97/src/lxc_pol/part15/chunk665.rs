//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 665/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk665(t1882: f64, t5332: f64, t5323: f64, t5319: f64, t5374: f64, t870: f64, t5315: f64, t5419: f64, t5381: f64, t5403: f64, t5399: f64, t5395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19318 = t1882 * t5332;
    let t19320 = t1882 * t5323;
    let t19322 = t1882 * t5319;
    let t19333 = t5374 * t870;
    let t19343 = t1882 * t5315;
    let t19387 = t1882 * t5419;
    let t19389 = t1882 * t5381;
    let t19449 = t1882 * t5403;
    let t19451 = t1882 * t5399;
    let t19453 = t1882 * t5395;
    (t19318, t19320, t19322, t19333, t19343, t19387, t19389, t19449, t19451, t19453)
}
