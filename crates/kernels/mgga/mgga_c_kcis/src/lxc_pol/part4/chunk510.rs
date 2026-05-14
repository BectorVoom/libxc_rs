//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 510/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk510<F: Float>(t137: F, t2479: F, t161: F, t2484: F, t818: F, t815: F, t823: F, t2491: F, t2490: F, t774: F, t755: F, t2526: F, t159: F, t64: F, t158: F, t157: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2605 = t2479 * t137;
    let t2606 = t2605 * t161;
    let t2608 = t2484 * t818;
    let t2610 = t815 * t823;
    let t2612 = t161 * t2491;
    let t2613 = t2490 * t2612;
    let t2615 = t823 * t774;
    let t2616 = t755 * t2615;
    let t2618 = t161 * t2526;
    let t2619 = t755 * t2618;
    let t2621 = t159 * t64;
    let t2622 = 1.0 / t2621;
    let t2623 = t158 * t2622;
    let t2624 = t157 * t2623;
    (t2605, t2606, t2608, t2610, t2612, t2613, t2615, t2616, t2618, t2619, t2621, t2622, t2623, t2624)
}
