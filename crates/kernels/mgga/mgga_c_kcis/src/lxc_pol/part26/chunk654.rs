//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 654/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk654<F: Float>(t7311: F, t7315: F, t7319: F, t7323: F, t7330: F, t7333: F, t7336: F, t7339: F, t7383: F, t7387: F, t7390: F, t7394: F) -> F {
    let t7396 = -t7311 / F::new(72.0) + t7315 / F::new(24.0) - t7319 / F::new(128.0) - t7323 / F::new(256.0) - F::new(19.0) / F::new(144.0) * t7330 + t7333 / F::new(18.0) + t7336 / F::new(3.0) - t7339 / F::new(12.0) + t7383 / F::new(16.0) + F::new(11.0) / F::new(18.0) * t7387 - F::new(2.0) / F::new(9.0) * t7390 + t7394 / F::new(8.0);
    t7396
}
