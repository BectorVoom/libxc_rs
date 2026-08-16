//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 637/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk637<F: Float>(t1189: F, t3477: F, t3440: F, t3445: F, t3449: F, t3453: F, t3455: F, t3457: F, t3461: F, t3467: F, t3469: F, t3471: F, t3475: F) -> (F, F) {
    let t3478 = t3477 * t1189;
    let t3480 = t3440 / F::cast_from(96.0_f64) - t3445 / F::cast_from(128.0_f64) - t3449 / F::cast_from(192.0_f64) + t3453 / F::cast_from(256.0_f64) + t3455 / F::cast_from(24.0_f64) - t3457 / F::cast_from(96.0_f64) - t3461 / F::cast_from(16.0_f64) + t3467 / F::cast_from(8.0_f64) - t3469 / F::cast_from(3.0_f64) + t3471 / F::cast_from(12.0_f64) - t3475 / F::cast_from(8.0_f64) + t3478 / F::cast_from(128.0_f64);
    (t3478, t3480)
}
