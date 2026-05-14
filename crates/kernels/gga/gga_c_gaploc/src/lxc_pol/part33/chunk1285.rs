//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1285/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1285<F: Float>(t12312: F, t29434: F, t29437: F, t29441: F, t29445: F, t29447: F, t29450: F, t29453: F, t29455: F, t29457: F, t32493: F, t32511: F, t32517: F, t32520: F, t5524: F, t32522: F, t32524: F, t32526: F, t32529: F, t32532: F, t32535: F, t32539: F, t32541: F, t32543: F, t32545: F, t32548: F, t32553: F, t32555: F, t32557: F) -> (F, F) {
    let t39438 = t32493 + 0.8545029144602471425e-3 * t5524 * t12312 - t29434 - t29437 - t29441 - t29445 - t29447 + t29450 + t29453 + t29455 - t29457 + t32511 + t32517 - t32520;
    let t39440 = -t32522 - t32524 - t32526 - t32529 + t32532 + t32535 + t32539 + t32541 + t32543 + t32545 - t32548 + t32553 + t32555 + t32557;
    (t39438, t39440)
}
