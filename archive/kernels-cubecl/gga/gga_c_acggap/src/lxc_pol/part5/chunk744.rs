//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 744/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk744<F: Float>(t1459: F, t398: F, t5537: F, t1008: F, t1851: F, t1298: F, t513: F, t1089: F, t1095: F, t1524: F, t495: F, t1856: F) -> (F, F, F, F, F, F, F) {
    let t5539 = t398 * t1459 * t5537;
    let t5542 = t1008 * t1851;
    let t5544 = t1298 * t513;
    let t5546 = t1089 * t1095 * t5544;
    let t5549 = t495 * t1524;
    let t5551 = t1089 * t1095 * t5549;
    let t5554 = t1008 * t1856;
    (t5539, t5542, t5544, t5546, t5549, t5551, t5554)
}
