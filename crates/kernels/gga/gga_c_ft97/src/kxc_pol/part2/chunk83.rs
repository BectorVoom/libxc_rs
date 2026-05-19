//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 83/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk83<F: Float>(t12: F, t213: F, t205: F, t206: F, t52: F, t204: F, t209: F, t41: F, rho1: F) -> (F, F, F, F, F, F) {
    let t214 = t12 * t213;
    let t215 = t205 * t205;
    let t216 = t215 * rho1;
    let t218 = F::new(1.0) / t206 / t216;
    let t220 = t52 * t214 * t218;
    let t221 = F::cast_from(0.55569193573523559258e-3_f64) * t220;
    let t222 = F::new(1.0) + F::cast_from(0.45058854638888888889e-1_f64) * t41 * t204 * t209 + t221;
    let t223 = t222 * t222;
    (t214, t215, t220, t221, t222, t223)
}
