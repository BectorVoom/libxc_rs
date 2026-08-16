//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1025/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1025<F: Float>(t137: F, t14423: F, t1165: F, t5012: F, t7564: F, t1181: F, t30209: F, t5099: F, t604: F, t4342: F, t7575: F, t8600: F) -> (F, F, F) {
    let t34248 = t14423 * t137;
    let t34251 = t7564 * t1165 * t34248 * t5012;
    let t34255 = t30209 * t1181 * t604 * t5099;
    let t34259 = t7575 * t1165 * t8600 * t4342;
    (t34251, t34255, t34259)
}
