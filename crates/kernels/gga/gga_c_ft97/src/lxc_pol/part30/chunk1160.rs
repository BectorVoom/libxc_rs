//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1160/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1160<F: Float>(t143276: F, t143321: F, t143324: F, t143327: F, t143500: F, t153388: F, t153390: F, t153395: F, t153399: F, t153402: F, t153405: F, t153414: F, t153418: F, t153422: F, t153427: F, t153431: F) -> F {
    let t154204 = -t143500 - t153388 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t153390 + t153395 / F::cast_from(6.0_f64) - t153399 + t153402 / F::cast_from(3.0_f64) - t153405 / F::cast_from(3.0_f64) - t143276 + F::cast_from(2.0_f64) * t143321 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t143324 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t143327 - F::cast_from(20.0_f64) * t153414 + F::cast_from(8.0_f64) * t153418 - F::cast_from(6.0_f64) * t153422 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t153427 + F::cast_from(3.0_f64) * t153431;
    t154204
}
