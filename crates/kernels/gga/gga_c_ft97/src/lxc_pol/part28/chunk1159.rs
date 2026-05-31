//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1159/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1159<F: Float>(t148270: F, t148275: F, t148278: F, t148282: F, t148286: F, t148290: F, t148295: F, t148299: F, t148304: F, t148309: F, t148311: F, t148315: F, t148319: F, t148323: F, t148327: F, t148331: F) -> F {
    let t148750 = -F::cast_from(6.0_f64) * t148270 - F::cast_from(15.0_f64) / F::cast_from(4.0_f64) * t148275 - t148278 / F::cast_from(3.0_f64) - t148282 - t148286 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t148290 - F::cast_from(20.0_f64) * t148295 + F::cast_from(8.0_f64) * t148299 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t148304 + t148309 / F::cast_from(6.0_f64) + t148311 / F::cast_from(9.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t148315 - t148319 / F::cast_from(12.0_f64) + F::cast_from(8.0_f64) * t148323 + t148327 / F::cast_from(3.0_f64) - t148331 / F::cast_from(2.0_f64);
    t148750
}
