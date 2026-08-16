//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1161/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1161<F: Float>(t29459: F, t29461: F, t29463: F, t29466: F, t29468: F, t29471: F, t29473: F, t29475: F, t29477: F, t29480: F, t29482: F, t29484: F) -> F {
    let t29486 = t29459 / F::cast_from(128.0_f64) + F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t29461 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t29463 - t29466 / F::cast_from(16.0_f64) - t29468 / F::cast_from(8.0_f64) - t29471 / F::cast_from(72.0_f64) - t29473 / F::cast_from(288.0_f64) + t29475 / F::cast_from(16.0_f64) - t29477 / F::cast_from(96.0_f64) + t29480 / F::cast_from(24.0_f64) - t29482 / F::cast_from(3.0_f64) + t29484 / F::cast_from(12.0_f64);
    t29486
}
