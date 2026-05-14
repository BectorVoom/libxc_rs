//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 559/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk559<F: Float>(t3300: F, t398: F, t5527: F, t1782: F, t372: F, t5011: F, t1524: F, t513: F, t1459: F, t1008: F, t1851: F, t1298: F, t1089: F, t1095: F, t495: F, t1856: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5529 = t398 * t3300 * t5527;
    let t5532 = t1782 * t372;
    let t5534 = t398 * t5011 * t5532;
    let t5537 = t513 * t1524;
    let t5539 = t398 * t1459 * t5537;
    let t5542 = t1008 * t1851;
    let t5544 = t1298 * t513;
    let t5546 = t1089 * t1095 * t5544;
    let t5549 = t495 * t1524;
    let t5551 = t1089 * t1095 * t5549;
    let t5554 = t1008 * t1856;
    (t5529, t5532, t5534, t5537, t5539, t5542, t5544, t5546, t5549, t5551, t5554)
}
