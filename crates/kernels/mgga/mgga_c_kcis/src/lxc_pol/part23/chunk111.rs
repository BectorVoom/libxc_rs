//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 111/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk111<F: Float>(t286: F, t499: F, t493: F) -> (F, F, F, F) {
    let t500 = t286 * t499;
    let t503 = F::cast_from(1.0_f64) + t493 * t500 / F::cast_from(96.0_f64);
    let t504 = F::ln(t503);
    let t506 = F::cast_from(1.0_f64) + F::cast_from(0.66725e-1_f64) * t504;
    let t507 = F::cast_from(1.0_f64) / t506;
    (t500, t503, t506, t507)
}
