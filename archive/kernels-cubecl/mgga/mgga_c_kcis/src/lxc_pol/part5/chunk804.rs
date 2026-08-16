//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 804/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk804<F: Float>(t2947: F, t4612: F, t6328: F, t6332: F, t6336: F) -> F {
    let t6338 = t2947 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4612 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t6328 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6332 - t6336 / F::cast_from(3.0_f64);
    t6338
}
