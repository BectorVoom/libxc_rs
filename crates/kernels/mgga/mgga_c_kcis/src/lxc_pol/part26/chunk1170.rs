//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1170/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1170<F: Float>(t29459: F, t29461: F, t29463: F, t29466: F, t29468: F, t29471: F, t29473: F, t29475: F, t29477: F, t29480: F, t29482: F, t29484: F) -> F {
    let t29651 = F::new(0.20234375e-1) * t29459 + F::cast_from(0.91666666666666666667e0_f64) * t29461 - F::cast_from(0.33333333333333333334e0_f64) * t29463 - F::new(0.9375e-1) * t29466 - F::new(0.1875e0) * t29468 - F::cast_from(0.20833333333333333333e-1_f64) * t29471 - F::cast_from(0.89930555555555555557e-2_f64) * t29473 + F::new(0.9375e-1) * t29475 - F::cast_from(0.26979166666666666667e-1_f64) * t29477 + F::new(0.625e-1) * t29480 - F::new(0.5e0) * t29482 + F::new(0.125e0) * t29484;
    t29651
}
