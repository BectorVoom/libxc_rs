//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1065/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1065<F: Float>(t141370: F, t141384: F, t151001: F, t151004: F, t151008: F, t151011: F, t151014: F, t151017: F, t151020: F, t151025: F, t151027: F, t151030: F, t151033: F, t151035: F, t151040: F, t151043: F) -> F {
    let t151344 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t151001 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t151004 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t151008 - t151011 / F::cast_from(9.0_f64) + t151014 / F::cast_from(27.0_f64) + t151017 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t151020 + t151025 / F::cast_from(3.0_f64) - t151027 / F::cast_from(9.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t151030 - t151033 / F::cast_from(3.0_f64) - t151035 / F::cast_from(36.0_f64) + t151040 / F::cast_from(12.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t151043 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t141370 + t141384 / F::cast_from(27.0_f64);
    t151344
}
