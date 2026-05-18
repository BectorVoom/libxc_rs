//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1230/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1230<F: Float>(t40363: F, t40365: F, t40368: F, t40370: F, t40373: F, t40377: F, t40381: F, t40386: F, t40396: F, t40400: F, t40404: F, t40406: F, t40408: F, t40415: F, t40419: F, t40423: F, t40432: F) -> F {
    let t40729 = -t40363 - t40365 + t40368 + t40370 - t40373 + t40377 + t40381 - t40386 + t40396 - t40400 - t40404 + t40406 - t40408 - t40415 - t40419 + t40423 + t40432;
    t40729
}
