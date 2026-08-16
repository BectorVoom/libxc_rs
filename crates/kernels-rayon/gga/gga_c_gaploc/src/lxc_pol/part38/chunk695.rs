//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 695/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk695(t13378: f64, t11430: f64, t901: f64, t1000: f64, t10497: f64, t12446: f64, t12450: f64, t13354: f64, t13356: f64, t13360: f64, t13365: f64, t13370: f64, t13374: f64, t13375: f64, t2859: f64, t574: f64) -> f64 {
    let t13379 = 0.14896037479937677779e-1_f64 * t13378;
    let t13380 = t11430 * t901;
    let t13381 = 0.14896037479937677779e-1_f64 * t13380;
    let t13382 = t13354 + t13356 + 0.71500979903700853338e0_f64 * t1000 * t10497 - 0.92023022289409799224e1_f64 * t574 * t13360 + t13365 - 0.63904876589867916126e-1_f64 * t12446 + 0.63904876589867916126e-1_f64 * t12450 - t13370 - t13374 - 0.21450293971110256002e1_f64 * t2859 * t13375 + t13379 + t13381;
    t13382
}
