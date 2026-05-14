//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1125/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1125<F: Float>(t22522: F, t22572: F, t29498: F, t422: F, t4474: F, t37482: F, t4441: F, t25752: F, t16863: F, t25754: F, t420: F, t3066: F, t925: F, t100580: F, t100610: F, t100613: F, t100618: F, t100667: F, t100784: F, t115440: F, t22541: F, t22585: F, t363: F, t37452: F, t37487: F, t379: F, t383: F, t401: F, t423: F, t4431: F, t5570: F, t73: F, t92531: F, t92639: F, t92873: F, t930: F, t93122: F, t93136: F, t93153: F) -> (F,) {
    let t115529 = t22522 * t22572 * t29498;
    let t115531 = t422 * t4474;
    let t115541 = t37482 * t4441;
    let t115542 = t115541 * t25752;
    let t115554 = t25754 * t420 * t16863;
    let t115567 = t925 * t3066;
    let t115576 = 0.85124811172839506173e-2 * t115529 - 0.38306165027777777777e-1 * t92873 * t5570 * t115531 * t379 - 0.12768721675925925926e-1 * t22541 * t5570 * t423 * t4431 * t401 - 0.30697322007724579005e-7 * t115542 * t92639 - 0.29673063867321838428e-4 * t93153 * t73 * t115440 - 0.39591381038172075259e-3 * t100610 + 0.35200977868053026979e-5 * t100613 - 0.98910212891072794758e-5 * t100618 + 0.31073410497668637766e-5 * t37452 * t383 * t25752 * t115554 - t92531 + 0.25876656037945937584e-6 * t37487 * t383 * t25752 * t115554 + 0.85124811172839506172e-2 * t100667 - 0.29693535778629056444e-3 * t93122 * t22585 * t930 * t100580 * t363 - 0.29693535778629056444e-3 * t93122 * t100784 * t115567 + 0.29693535778629056444e-3 * t93136 * t22585 * t930 * t925 * t401;
    (t115576,)
}
