//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1084/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1084<F: Float>(t22735: F, t7837: F, t45566: F, t5576: F, t22833: F, t66: F, t22564: F, t5532: F, t77: F, t1614: F, t1602: F, t409: F, t39: F, t5585: F, t1613: F, t5589: F) -> (F, F, F, F, F, F, F, F) {
    let t92314 = t7837 * t22735;
    let t92327 = t45566 * t5576;
    let t92335 = t22833 * t66;
    let t92336 = t92335 * t22564;
    let t92339 = t77 * t5532;
    let t92341 = t7837 * t92339 * t1614;
    let t92353 = t1602 * t409;
    let t92354 = t39 * t5585;
    let t92356 = t92354 * t1613 * t5589;
    (t92314, t92327, t92336, t92339, t92341, t92353, t92354, t92356)
}
