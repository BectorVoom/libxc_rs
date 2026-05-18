//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 107/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk107<F: Float>(t12: F, t205: F, t206: F, t52: F, t204: F, t209: F, t41: F, t203: F, rho1: F, sigma2: F) -> (F, F, F, F) {
    let t213 = sigma2 * sigma2;
    let t214 = t12 * t213;
    let t215 = t205 * t205;
    let t216 = t215 * rho1;
    let t218 = F::new(1.0) / t206 / t216;
    let t220 = t52 * t214 * t218;
    let t221 = F::new(0.55569193573523559258e-3) * t220;
    let t222 = F::new(1.0) + F::new(0.45058854638888888889e-1) * t41 * t204 * t209 + t221;
    let t223 = t222 * t222;
    let t224 = t203 * t223;
    (t220, t221, t223, t224)
}
