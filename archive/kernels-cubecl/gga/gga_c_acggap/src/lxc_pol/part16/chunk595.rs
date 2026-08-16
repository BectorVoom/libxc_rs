//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 595/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk595<F: Float>(t1861: F, t997: F, t1851: F, t1856: F, t1894: F, t336: F, t372: F, t4630: F, t495: F, t1298: F, t1501: F, t1143: F, t1734: F) -> (F, F, F, F, F, F, F) {
    let t5579 = t997 * t1861;
    let t5581 = t997 * t1851;
    let t5583 = t997 * t1856;
    let t5586 = t336 * t1894 * t372;
    let t5590 = t336 * t4630 * t495;
    let t5594 = t336 * t1501 * t1298;
    let t5598 = t336 * t1143 * t1734;
    (t5579, t5581, t5583, t5586, t5590, t5594, t5598)
}
