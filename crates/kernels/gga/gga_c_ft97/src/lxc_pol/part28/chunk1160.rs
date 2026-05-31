//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1160/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1160<F: Float>(t139241: F, t139254: F, t139257: F, t139275: F, t139278: F, t148334: F, t148338: F, t148342: F, t148346: F, t148349: F, t148353: F, t148360: F, t148365: F, t148369: F, t148373: F, t148375: F) -> F {
    let t148765 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t148334 - t148338 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t139241 - t148342 / F::cast_from(3.0_f64) - t148346 / F::cast_from(12.0_f64) + t148349 / F::cast_from(6.0_f64) - t148353 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t139254 - t139257 - t139275 / F::cast_from(12.0_f64) + t139278 / F::cast_from(6.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t148360 + t148365 / F::cast_from(4.0_f64) + F::cast_from(12.0_f64) * t148369 - F::cast_from(6.0_f64) * t148373 - t148375 / F::cast_from(18.0_f64);
    t148765
}
