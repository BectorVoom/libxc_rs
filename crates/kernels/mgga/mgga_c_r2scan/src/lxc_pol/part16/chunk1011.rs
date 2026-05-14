//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1011/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1011<F: Float>(t42383: F, t42387: F, t42391: F, t42395: F, t42398: F, t42402: F, t42405: F, t42408: F, t42411: F, t42415: F, t42417: F, t42422: F, t42427: F, t42431: F, t42435: F, t42437: F) -> (F,) {
    let t42438 = -t42383 - t42387 + t42391 + t42395 - t42398 + t42402 + t42405 + t42408 + t42411 + t42415 - t42417 - t42422 + t42427 + t42431 - t42435 - t42437;
    (t42438,)
}
