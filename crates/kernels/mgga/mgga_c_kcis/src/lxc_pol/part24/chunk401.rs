//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 401/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk401<F: Float>(t154: F, t2491: F, t2593: F, t774: F, t812: F, t808: F, t2526: F, t153: F, t2150: F, t137: F, t2479: F, t161: F, t2484: F, t818: F, t815: F, t823: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2594 = t154 * t2491;
    let t2595 = t2593 * t2594;
    let t2597 = t812 * t774;
    let t2598 = t808 * t2597;
    let t2600 = t154 * t2526;
    let t2601 = t808 * t2600;
    let t2603 = t153 * t2150;
    let t2605 = t2479 * t137;
    let t2606 = t2605 * t161;
    let t2608 = t2484 * t818;
    let t2610 = t815 * t823;
    (t2594, t2595, t2597, t2598, t2600, t2601, t2603, t2605, t2606, t2608, t2610)
}
