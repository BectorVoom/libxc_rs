//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1308/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1308<F: Float>(t25462: F, t31653: F, t28924: F, t4246: F, t31658: F, t19378: F, t25188: F, t19435: F, t1466: F, t31682: F, t681: F, t111743: F, t25412: F, t25459: F, t28870: F, t28955: F, t28972: F, t29008: F, t29026: F, t31344: F, t31677: F, t31688: F, t6210: F, t6216: F, t684: F, t6963: F) -> (F, F, F, F) {
    let t125554 = t25462 * t31653;
    let t125560 = t4246 * t28924;
    let t125564 = t25462 * t31658;
    let t125572 = t25188 * t19378;
    let t125574 = t25188 * t19435;
    let t125581 = t1466 * t681 * t31682;
    let t125583 = -t125554 / 27.0 - 2.0 / 3.0 * t6963 * t28870 - 2.0 / 3.0 * t6963 * t28972 - 4.0 * t125560 - t25459 * t31344 / 9.0 + t111743 - 2.0 / 27.0 * t125564 + t6216 * t25412 * t31677 * t684 / 9.0 - t29008 * t29026 / 9.0 + 4.0 * t125572 + 8.0 * t125574 + t6963 * t28955 / 3.0 - 2.0 / 3.0 * t6210 * t31688 + 2.0 / 9.0 * t125581;
    (t125560, t125572, t125574, t125583)
}
