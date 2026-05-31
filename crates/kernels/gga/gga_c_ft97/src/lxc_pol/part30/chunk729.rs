//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 729/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk729<F: Float>(t33332: F, t743: F, t193: F, t6109: F, t33286: F, t33291: F, t33297: F, t33305: F, t33310: F, t33314: F, t33318: F, t33322: F, t33326: F, t33330: F) -> (F, F, F) {
    let t33333 = t743 * t33332;
    let t33335 = t6109 * t193 * t33333;
    let t33337 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t33286 + t33291 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33297 + F::cast_from(4.0_f64) * t33305 - F::cast_from(2.0_f64) * t33310 - t33314 / F::cast_from(2.0_f64) - t33318 - t33322 / F::cast_from(3.0_f64) - F::cast_from(3.0_f64) * t33326 + F::cast_from(2.0_f64) * t33330 + t33335 / F::cast_from(4.0_f64);
    (t33333, t33335, t33337)
}
