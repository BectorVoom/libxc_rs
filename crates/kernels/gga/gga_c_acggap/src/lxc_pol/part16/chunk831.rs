//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 831/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk831<F: Float>(t30594: F, t580: F, t3151: F, t56: F, t569: F, t571: F, t31276: F, t7382: F, t1072: F, t429: F, t7507: F, t7512: F, t310: F, t7506: F, t7514: F, t7518: F) -> (F, F, F, F, F, F) {
    let t31376 = t30594 * t580;
    let t31377 = 1309.0 / 5184.0 * t31376;
    let t31380 = t3151 * t56 * t569 * t571;
    let t31381 = 455.0 / 1296.0 * t31380;
    let t31382 = t31276 * t7382;
    let t31386 = t7507 * t7512 * t429 * t1072;
    let t31388 = t310 * t7506;
    let t31389 = t31388 * t7514;
    let t31390 = 0.12862205435420921092e-2 * t31389;
    let t31391 = t31388 * t7518;
    (t31377, t31381, t31382, t31386, t31390, t31391)
}
