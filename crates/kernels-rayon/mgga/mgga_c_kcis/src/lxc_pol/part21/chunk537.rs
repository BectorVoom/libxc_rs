//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 537/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk537(t2815: f64, t3438: f64, t3437: f64, t3226: f64, t381: f64, t3228: f64, t388: f64, t387: f64, t3190: f64, t358: f64, t382: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3439 = t3438 * t2815;
    let t3440 = t3437 * t3439;
    let t3442 = t3226 * t381;
    let t3443 = t388 * t3228;
    let t3444 = t387 * t3443;
    let t3445 = t3442 * t3444;
    let t3447 = t358 * t3190;
    let t3448 = t387 * t3447;
    let t3449 = t382 * t3448;
    let t3451 = t388 * t3316;
    (t3439, t3440, t3444, t3445, t3448, t3449, t3451)
}
