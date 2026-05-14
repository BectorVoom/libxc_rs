//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1119/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1119<F: Float>(t3051: F, t6108: F, t27883: F, t681: F, t89: F, t2371: F, t27742: F, t2399: F, t6109: F, t6879: F, t27832: F, t27828: F, t1882: F, t27860: F, t27863: F, t27866: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t108310 = t6108 * t3051;
    let t108333 = t89 * t681 * t27883;
    let t108334 = 4.0 / 3.0 * t108333;
    let t108335 = t2371 * t27742;
    let t108345 = t6109 * t2399 * t6879;
    let t108353 = t89 * t681 * t27832;
    let t108354 = 4.0 / 3.0 * t108353;
    let t108356 = t89 * t681 * t27828;
    let t108357 = 4.0 / 3.0 * t108356;
    let t108393 = t1882 * t27860;
    let t108394 = 4.0 / 9.0 * t108393;
    let t108429 = t1882 * t27863;
    let t108430 = 4.0 / 9.0 * t108429;
    let t108431 = t1882 * t27866;
    (t108310, t108333, t108334, t108335, t108345, t108353, t108354, t108356, t108357, t108393, t108394, t108429, t108430, t108431)
}
