//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1224/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1224<F: Float>(t103172: F, t27369: F, t102529: F, t102548: F, t103069: F, t27459: F, t28344: F, t28369: F, t29259: F, t7908: F, t94614: F, t98119: F, t98632: F, t98637: F, t98649: F, t98652: F) -> (F,) {
    let t103613 = t27369 * t103172;
    let t103624 = -t94614 - 0.27802083333333333334e-2 * t7908 * t103069 - 0.22109259259259259259e-2 * t102529 - 0.6183646701388888889e-4 * t103613 - 0.18550940104166666667e-3 * t98119 * t28344 + 0.46336805555555555556e-3 * t27459 * t29259 - 0.13901041666666666667e-2 * t28369 * t28344 + 0.22109259259259259259e-2 * t102548 - 0.44218518518518518516e-2 * t98632 + 0.22109259259259259259e-2 * t98637 + t98649 + t98652;
    (t103624,)
}
