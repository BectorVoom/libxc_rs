//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 878/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk878(t13531: f64, t13572: f64, t13577: f64, t13582: f64, t13586: f64, t13589: f64, t13651: f64, t13656: f64, t13659: f64, t13662: f64, t13666: f64, t224: f64, t2387: f64, t3723: f64, t3726: f64, t678: f64, t680: f64, t695: f64, t9533: f64, t9543: f64) -> f64 {
    let t13669 = -0.33776098467676728323e-5_f64 * t13531 * t3726 - 0.11627450473218896e-1_f64 * t678 * t680 * t13572 + 0.67598802253579164263e-4_f64 * t13577 * t3726 + 0.13519760450715832853e-3_f64 * t9543 * t13582 - 0.67598802253579164263e-4_f64 * t9543 * t13586 - 0.40559281352147498558e-4_f64 * t13589 * t3726 - t224 * t695 * t13651 - 0.13519760450715832853e-3_f64 * t3723 * t13656 - 0.23254900946437792e-2_f64 * t2387 * t13659 - 0.279058811357253504e-2_f64 * t678 * t13662 - 0.23254900946437792e-1_f64 * t9533 * t13666;
    t13669
}
