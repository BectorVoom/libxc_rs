//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 728/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk728<F: Float>(t42898: F, t37275: F, t921: F, t2497: F, t3553: F, t4349: F, t27232: F, t3366: F, t13343: F, t17288: F, t13350: F, t605: F, t1382: F, t3599: F, t11298: F, t19933: F) -> (F, F, F, F, F, F, F, F) {
    let t44665 = 0.47425011059460249332e-2 * t42898;
    let t44671 = t37275 * t921;
    let t44674 = 6.0 * t4349 * t3553 * t2497;
    let t44676 = 4.0 * t27232 * t3366;
    let t44678 = 6.0 * t17288 * t13343;
    let t44684 = 6.0 * t4349 * t13350 * t605;
    let t44687 = 2.0 * t1382 * t3599 * t2497;
    let t44689 = 6.0 * t19933 * t11298;
    (t44665, t44671, t44674, t44676, t44678, t44684, t44687, t44689)
}
