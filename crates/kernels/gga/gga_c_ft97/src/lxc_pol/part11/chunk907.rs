//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 907/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk907<F: Float>(t38503: F, t38547: F, t38594: F, t38631: F, t457: F, t91: F, t37427: F, t37433: F, t38257: F, t38260: F, t38266: F, t38271: F, t38275: F, t38279: F, t38281: F, t38285: F, t38288: F, t38292: F, t38449: F, t38459: F) -> (F, F) {
    let t38635 = t91 * t457 * (t38503 + t38547 + t38594 + t38631);
    let t38637 = F::cast_from(8.0_f64) * t37427 + F::cast_from(24.0_f64) * t37433 - t38257 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38260 + F::cast_from(8.0_f64) * t38266 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38271 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t38275 - F::cast_from(4.0_f64) * t38279 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t38281 - F::cast_from(4.0_f64) * t38285 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t38288 - F::cast_from(8.0_f64) * t38292 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t38449 - F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t38459 + t38635 / F::cast_from(2.0_f64);
    (t38635, t38637)
}
