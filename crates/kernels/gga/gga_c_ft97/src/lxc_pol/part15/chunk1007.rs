//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1007/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1007<F: Float>(t378: F, t85538: F, t92: F, t58969: F, t73956: F, t73958: F, t73983: F, t85518: F, t85522: F, t85526: F, t85529: F, t85533: F, t85536: F) -> (F, F) {
    let t85540 = t92 * t378 * t85538;
    let t85542 = -F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t58969 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t73956 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t73958 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t85518 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t73983 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t85522 - F::cast_from(8.0_f64) * t85526 + F::cast_from(8.0_f64) * t85529 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t85533 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t85536 + F::cast_from(8.0_f64) * t85540;
    (t85540, t85542)
}
