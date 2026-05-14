//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 759/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk759<F: Float>(t25: F, t695: F, t677: F, t1095: F, t2393: F, t2426: F, t3817: F, t13443: F, t13444: F, t13449: F, t13453: F, t13456: F, t13460: F, t13464: F, t13468: F, t13469: F, t1701: F, t2387: F, t2388: F, t2389: F, t2455: F, t3766: F, t3767: F, t3789: F, t3790: F, t678: F, t709: F) -> (F,) {
    let t13473 = t695 * t25;
    let t13474 = t677 * t13473;
    let t13475 = t2393 * t1095;
    let t13479 = t2426 * t3817;
    let t13489 = 0.11854761295685025975e-1 * t13443 * t1701 * t13444 + 0.46509801892875584e-2 * t678 * t13449 + 0.23254900946437792e-2 * t678 * t13453 - 0.11619434043764639964e-3 * t678 * t13456 + 0.11627450473218896e-1 * t2387 * t13460 + 0.46509801892875584e-1 * t13464 * t2389 - 0.38731446812548799882e-3 * t13468 * t13469 * t2388 - 0.46509801892875584e-2 * t13474 * t13475 * t2388 + 4.0 * t3789 * t13479 * t709 + 2.0 * t3789 * t3790 * t2455 - 2.0 * t3766 * t3767 * t2455;
    (t13489,)
}
