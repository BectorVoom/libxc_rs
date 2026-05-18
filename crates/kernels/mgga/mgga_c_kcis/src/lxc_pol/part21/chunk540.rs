//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 540/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk540<F: Float>(t1189: F, t3477: F, t3440: F, t3445: F, t3449: F, t3453: F, t3455: F, t3457: F, t3461: F, t3467: F, t3469: F, t3471: F, t3475: F) -> (F, F) {
    let t3478 = t3477 * t1189;
    let t3480 = t3440 / F::new(96.0) - t3445 / F::new(128.0) - t3449 / F::new(192.0) + t3453 / F::new(256.0) + t3455 / F::new(24.0) - t3457 / F::new(96.0) - t3461 / F::new(16.0) + t3467 / F::new(8.0) - t3469 / F::new(3.0) + t3471 / F::new(12.0) - t3475 / F::new(8.0) + t3478 / F::new(128.0);
    (t3478, t3480)
}
