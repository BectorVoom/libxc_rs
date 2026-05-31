//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 886/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk886<F: Float>(t7311: F, t7315: F, t7319: F, t7323: F, t7330: F, t7333: F, t7336: F, t7339: F, t7383: F, t7387: F, t7390: F, t7394: F) -> F {
    let t7396 = -t7311 / F::cast_from(72.0_f64) + t7315 / F::cast_from(24.0_f64) - t7319 / F::cast_from(128.0_f64) - t7323 / F::cast_from(256.0_f64) - F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t7330 + t7333 / F::cast_from(18.0_f64) + t7336 / F::cast_from(3.0_f64) - t7339 / F::cast_from(12.0_f64) + t7383 / F::cast_from(16.0_f64) + F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t7387 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t7390 + t7394 / F::cast_from(8.0_f64);
    t7396
}
