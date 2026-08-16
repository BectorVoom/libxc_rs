//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 409/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk409(t3358: f64, t492: f64, t105: f64, t3124: f64, t3132: f64, t3329: f64, t3341: f64, t3346: f64, t3349: f64, t3353: f64, t3357: f64, t1016: f64, t921: f64) -> (f64, f64, f64) {
    let t3359 = t492 * t3358;
    let t3362 = t3329 + 0.28455006635676149599e-1_f64 * t105 * t3341 + t3346 - t3349 + t3124 - t3132 - t3353 + t3357 - 0.28455006635676149599e-1_f64 * t105 * t3359;
    let t3366 = t1016 * t921;
    (t3359, t3362, t3366)
}
