//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 402/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk402<F: Float>(t2568: F, t734: F, t2441: F, t642: F, t735: F, t2507: F, t716: F) -> (F, F, F, F, F) {
    let t2569 = t734 * t2568;
    let t2571 = t642 * t2441;
    let t2572 = t735 * t2571;
    let t2573 = t734 * t2572;
    let t2575 = t2507 * t716;
    (t2569, t2571, t2572, t2573, t2575)
}
