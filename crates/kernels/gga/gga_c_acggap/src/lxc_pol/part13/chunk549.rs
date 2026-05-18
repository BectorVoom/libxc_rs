//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 549/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk549<F: Float>(t118: F, t3644: F, t150: F, t164: F, t177: F, t360: F, t864: F, t368: F, t398: F, t1036: F, t372: F, t1095: F) -> (F, F, F, F, F, F, F, F) {
    let t3645 = t3644 * t118;
    let t3646 = t3645 * t150;
    let t3649 = F::new(0.21437009059034868486e-3) * t3646 * t164 * t177;
    let t3650 = t864 * t360;
    let t3652 = t398 * t368 * t3650;
    let t3653 = t1036 * t3652;
    let t3655 = t864 * t372;
    let t3657 = t398 * t1095 * t3655;
    (t3645, t3646, t3649, t3650, t3652, t3653, t3655, t3657)
}
