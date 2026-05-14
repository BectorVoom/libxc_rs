//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 612/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk612<F: Float>(t13395: F, t3785: F, t13434: F, t9609: F, t1096: F, t2428: F, t680: F, t13531: F, t13572: F, t13577: F, t13582: F, t13586: F, t13589: F, t13651: F, t13656: F, t224: F, t2387: F, t3723: F, t3726: F, t678: F, t695: F, t9533: F, t9543: F) -> (F, F, F, F) {
    let t13659 = t3785 * t13395;
    let t13662 = t9609 * t13434;
    let t13666 = t680 * t1096 * t2428;
    let t13669 = -0.33776098467676728323e-5 * t13531 * t3726 - 0.11627450473218896e-1 * t678 * t680 * t13572 + 0.67598802253579164263e-4 * t13577 * t3726 + 0.13519760450715832853e-3 * t9543 * t13582 - 0.67598802253579164263e-4 * t9543 * t13586 - 0.40559281352147498558e-4 * t13589 * t3726 - t224 * t695 * t13651 - 0.13519760450715832853e-3 * t3723 * t13656 - 0.23254900946437792e-2 * t2387 * t13659 - 0.279058811357253504e-2 * t678 * t13662 - 0.23254900946437792e-1 * t9533 * t13666;
    (t13659, t13662, t13666, t13669)
}
