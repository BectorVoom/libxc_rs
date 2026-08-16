//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 81/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk81<F: Float>(t200: F, t11: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t202 = F::cast_from(0.942486901e0_f64) + F::cast_from(0.349064173e0_f64) * t200;
    let t203 = t202 * t202;
    let t204 = t11 * sigma2;
    let t205 = rho1 * rho1;
    let t206 = pow_1_3::<F>(rho1);
    let t207 = t206 * t206;
    let t209 = F::cast_from(1.0_f64) / t207 / t205;
    (t202, t203, t204, t205, t206, t207, t209)
}
