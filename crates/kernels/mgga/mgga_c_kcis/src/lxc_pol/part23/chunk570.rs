//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 570/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk570<F: Float>(t3794: F, t3795: F, t5469: F, t5472: F, t5475: F, t5479: F) -> F {
    let t5481 = t3794 + t3795 / F::cast_from(9.0_f64) + t5469 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5472 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5475 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5479;
    t5481
}
