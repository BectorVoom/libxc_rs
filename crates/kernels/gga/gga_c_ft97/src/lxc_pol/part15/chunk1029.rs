//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1029/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1029<F: Float>(t1555: F, t1558: F, t85451: F, t89: F, t57491: F, t57527: F, t86199: F, t86202: F, t86205: F, t86208: F, t86211: F, t86214: F, t86217: F, t86220: F, t86223: F, t86226: F, t86232: F, t86236: F) -> (F, F) {
    let t86240 = t89 * t1555 * t1558 * t85451;
    let t86242 = -F::cast_from(8.0_f64) * t86199 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86202 + F::cast_from(2.0_f64) * t86205 + F::cast_from(8.0_f64) * t86208 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t86211 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86214 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t86217 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t86220 + F::cast_from(8.0_f64) * t86223 + F::cast_from(8.0_f64) * t86226 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t57491 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t57527 - F::cast_from(36.0_f64) * t86232 - F::cast_from(8.0_f64) * t86236 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t86240;
    (t86240, t86242)
}
