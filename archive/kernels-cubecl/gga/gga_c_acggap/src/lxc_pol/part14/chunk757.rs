//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 757/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk757<F: Float>(t2069: F, t8480: F, t2068: F, t137: F, t1524: F, t1089: F, t1459: F, t598: F, t355: F, t513: F, t7458: F, t1980: F) -> (F, F, F, F, F, F, F, F) {
    let t8481 = t8480 * t2069;
    let t8482 = t2068 * t8481;
    let t8484 = t137 * t1524;
    let t8486 = t1089 * t1459 * t8484;
    let t8487 = t598 * t8486;
    let t8489 = t355 * t513;
    let t8491 = t7458 * t1459 * t8489;
    let t8492 = t1980 * t8491;
    (t8481, t8482, t8484, t8486, t8487, t8489, t8491, t8492)
}
