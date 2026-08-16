//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 901/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk901(t3923: f64, t408: f64, t13438: f64, t1219: f64, t3729: f64, t1286: f64, t3283: f64, t3485: f64, t3484: f64, t3482: f64, t1056: f64) -> (f64, f64, f64, f64) {
    let t13440 = 1.0_f64 / t3923 / t408;
    let t13441 = t13438 * t13440;
    let t13448 = t3729 * t1219;
    let t13451 = t3283 * t1286;
    let t13452 = t3485 * t13451;
    let t13453 = t3484 * t13452;
    let t13454 = t3482 * t13453;
    let t13456 = t3283 * t1056;
    (t13441, t13448, t13454, t13456)
}
