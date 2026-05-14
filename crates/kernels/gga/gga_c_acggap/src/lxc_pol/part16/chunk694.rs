//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 694/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk694<F: Float>(t103: F, t566: F, t95: F, t1427: F, t7278: F, t2541: F, t5439: F, t1298: F, t469: F, t104: F, t2354: F, t1658: F, t609: F, t2147: F, t545: F, t618: F) -> (F, F, F, F, F, F, F, F) {
    let t8372 = t566 * t95 * t103;
    let t8373 = t7278 * t1427;
    let t8379 = t2541 * t5439;
    let t8382 = t469 * t1298;
    let t8387 = t104 * t2354;
    let t8392 = t609 * t1658;
    let t8393 = t2147 * t8392;
    let t8396 = t545 * t618;
    (t8372, t8373, t8379, t8382, t8387, t8392, t8393, t8396)
}
