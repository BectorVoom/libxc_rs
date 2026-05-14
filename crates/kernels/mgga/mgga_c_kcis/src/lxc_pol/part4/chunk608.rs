//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 608/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk608<F: Float>(t3440: F, t3445: F, t3449: F, t3453: F, t3455: F, t3457: F, t3461: F, t3467: F, t3469: F, t3471: F, t3475: F, t3478: F, t3435: F, t1142: F, t20: F, t2865: F) -> (F, F, F) {
    let t3480 = t3440 / 96.0 - t3445 / 128.0 - t3449 / 192.0 + t3453 / 256.0 + t3455 / 24.0 - t3457 / 96.0 - t3461 / 16.0 + t3467 / 8.0 - t3469 / 3.0 + t3471 / 12.0 - t3475 / 8.0 + t3478 / 128.0;
    let t3481 = t3435 + t3480;
    let t3482 = t1142 * t3481;
    let t3483 = t2865 * t20;
    (t3481, t3482, t3483)
}
