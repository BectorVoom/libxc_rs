//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 45/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk45<F: Float>(t214: F, t218: F, t52: F, t204: F, t209: F, t41: F) -> (F, F, F) {
    let t220 = t52 * t214 * t218;
    let t221 = F::cast_from(0.55569193573523559258e-3_f64) * t220;
    let t222 = F::new(1.0) + F::cast_from(0.45058854638888888889e-1_f64) * t41 * t204 * t209 + t221;
    (t220, t221, t222)
}
