//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 511/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk511<F: Float>(t2919: F, t2947: F, t4612: F, t4615: F, t4618: F, t4623: F) -> F {
    let t4625 = t2947 + t2919 / F::cast_from(9.0_f64) + t4612 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4615 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4618 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4623;
    t4625
}
