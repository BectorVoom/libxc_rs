//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 775/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk775<F: Float>(t8122: F, t8354: F, t105: F, t469: F, t103: F, t566: F, t95: F, t1298: F, t545: F, t618: F) -> (F, F, F, F, F, F) {
    let t8355 = t8122 + t8354;
    let t8356 = t105 * t8355;
    let t8357 = t8356 * t469;
    let t8372 = t566 * t95 * t103;
    let t8382 = t469 * t1298;
    let t8396 = t545 * t618;
    (t8355, t8356, t8357, t8372, t8382, t8396)
}
