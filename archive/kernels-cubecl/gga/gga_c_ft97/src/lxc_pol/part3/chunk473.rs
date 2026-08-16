//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 473/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk473<F: Float>(t3565: F, t605: F, t144: F, t1060: F, t558: F, t574: F, t1017: F, t616: F, t1045: F, t604: F) -> (F, F, F, F, F) {
    let t3566 = t605 * t3565;
    let t3567 = t144 * t3566;
    let t3571 = t574 * t1060 * t558;
    let t3575 = t574 * t616 * t1017;
    let t3578 = t1045 * t604;
    (t3566, t3567, t3571, t3575, t3578)
}
