//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 95/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk95<F: Float>(t2: F, t241: F, t192: F, t92: F, t91: F, t244: F) -> (F, F, F, F, F) {
    let t248 = t241 * t2;
    let t249 = t192 * t248;
    let t250 = t92 * t249;
    let t251 = F::sqrt(t250);
    let t252 = t91 * t251;
    let t255 = F::new(3.0) + t252 / F::new(3.0) + t244 / F::new(3.0);
    (t249, t250, t251, t252, t255)
}
