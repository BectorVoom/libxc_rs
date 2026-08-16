//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 865/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk865(t1113: f64, t695: f64, t3758: f64, t122: f64, t677: f64, t1095: f64, t2378: f64, t25: f64, t2393: f64, t2426: f64, t3817: f64, t13443: f64, t13444: f64, t13449: f64, t13453: f64, t13456: f64, t13460: f64, t1701: f64, t2387: f64, t2388: f64, t2389: f64, t2455: f64, t3766: f64, t3767: f64, t3789: f64, t3790: f64, t678: f64, t709: f64) -> f64 {
    let t13463 = t695 * t1113;
    let t13464 = t3758 * t13463;
    let t13467 = t695 * t122;
    let t13468 = t677 * t13467;
    let t13469 = t2378 * t1095;
    let t13473 = t695 * t25;
    let t13474 = t677 * t13473;
    let t13475 = t2393 * t1095;
    let t13479 = t2426 * t3817;
    let t13489 = 0.11854761295685025975e-1_f64 * t13443 * t1701 * t13444 + 0.46509801892875584e-2_f64 * t678 * t13449 + 0.23254900946437792e-2_f64 * t678 * t13453 - 0.11619434043764639964e-3_f64 * t678 * t13456 + 0.11627450473218896e-1_f64 * t2387 * t13460 + 0.46509801892875584e-1_f64 * t13464 * t2389 - 0.38731446812548799882e-3_f64 * t13468 * t13469 * t2388 - 0.46509801892875584e-2_f64 * t13474 * t13475 * t2388 + 4.0_f64 * t3789 * t13479 * t709 + 2.0_f64 * t3789 * t3790 * t2455 - 2.0_f64 * t3766 * t3767 * t2455;
    t13489
}
