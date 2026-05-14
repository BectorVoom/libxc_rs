//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 848/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk848<F: Float>(t31362: F, t7357: F, t7447: F, t7503: F, t30594: F, t580: F, t3151: F, t56: F, t569: F, t571: F, t31276: F, t7382: F, t1072: F, t429: F, t7507: F, t7512: F) -> (F, F, F, F, F, F) {
    let t31363 = t31362 * t7357;
    let t31374 = t7447 * t7503;
    let t31376 = t30594 * t580;
    let t31377 = 1309.0 / 5184.0 * t31376;
    let t31380 = t3151 * t56 * t569 * t571;
    let t31381 = 455.0 / 1296.0 * t31380;
    let t31382 = t31276 * t7382;
    let t31386 = t7507 * t7512 * t429 * t1072;
    (t31363, t31374, t31377, t31381, t31382, t31386)
}
