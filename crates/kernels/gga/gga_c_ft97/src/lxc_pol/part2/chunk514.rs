//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 514/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk514<F: Float>(t3565: F, t605: F, t144: F, t1060: F, t558: F, t574: F, t1017: F, t616: F, t1045: F, t604: F, t609: F, t1053: F, t2142: F, t2140: F, t2165: F, t2167: F, t28: F, t3480: F, t3485: F, t3489: F, t3541: F, t3545: F, t3548: F, t3551: F, t446: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3566 = t605 * t3565;
    let t3567 = t144 * t3566;
    let t3571 = t574 * t1060 * t558;
    let t3575 = t574 * t616 * t1017;
    let t3578 = t1045 * t604;
    let t3579 = t3578 * t609;
    let t3580 = t144 * t3579;
    let t3583 = t2142 * t1053;
    let t3584 = t144 * t3583;
    let t3587 = t2165 / 9.0 + t2167 / 9.0 - t2140 / 9.0 + t446 * t3480 / 3.0 + 2.0 / 3.0 * t446 * t3485 - t3489 / 9.0 + t89 * t28 * t3541 / 3.0 + t3545 / 9.0 - t446 * t3548 / 3.0 + t3551 / 9.0 - t446 * t3567 / 3.0 - t446 * t3571 / 3.0 - t446 * t3575 / 3.0 - t446 * t3580 / 3.0 - t446 * t3584 / 3.0;
    (t3566, t3567, t3571, t3575, t3578, t3579, t3580, t3583, t3584, t3587)
}
