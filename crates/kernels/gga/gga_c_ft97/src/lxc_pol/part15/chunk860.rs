//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 860/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk860<F: Float>(t7763: F, t81: F, t342: F, t344: F, t8639: F, t7800: F, t37292: F, t1586: F, t22: F, t36452: F, t37991: F, t96: F) -> (F, F, F, F, F) {
    let t38327 = t81 * t7763;
    let t38355 = F::cast_from(5.0_f64) / F::cast_from(54.0_f64) * t342 * t8639 * t344;
    let t38357 = t81 * t7800;
    let t38392 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t37292;
    let t38456 = F::cast_from(1.0_f64) / t96 / t37991 / t22 / t1586 / t36452 / F::cast_from(96.0_f64);
    (t38327, t38355, t38357, t38392, t38456)
}
