//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 878/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk878<F: Float>(t13531: F, t13572: F, t13577: F, t13582: F, t13586: F, t13589: F, t13651: F, t13656: F, t13659: F, t13662: F, t13666: F, t224: F, t2387: F, t3723: F, t3726: F, t678: F, t680: F, t695: F, t9533: F, t9543: F) -> F {
    let t13669 = -F::cast_from(0.33776098467676728323e-5_f64) * t13531 * t3726 - F::cast_from(0.11627450473218896e-1_f64) * t678 * t680 * t13572 + F::cast_from(0.67598802253579164263e-4_f64) * t13577 * t3726 + F::cast_from(0.13519760450715832853e-3_f64) * t9543 * t13582 - F::cast_from(0.67598802253579164263e-4_f64) * t9543 * t13586 - F::cast_from(0.40559281352147498558e-4_f64) * t13589 * t3726 - t224 * t695 * t13651 - F::cast_from(0.13519760450715832853e-3_f64) * t3723 * t13656 - F::cast_from(0.23254900946437792e-2_f64) * t2387 * t13659 - F::cast_from(0.279058811357253504e-2_f64) * t678 * t13662 - F::cast_from(0.23254900946437792e-1_f64) * t9533 * t13666;
    t13669
}
