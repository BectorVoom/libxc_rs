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
    let t85542 = -F::new(8.0) / F::new(9.0) * t58969 - F::new(8.0) / F::new(3.0) * t73956 + F::new(8.0) / F::new(9.0) * t73958 + F::new(40.0) / F::new(9.0) * t85518 + F::new(40.0) / F::new(81.0) * t73983 - F::new(20.0) / F::new(9.0) * t85522 - F::new(8.0) * t85526 + F::new(8.0) * t85529 - F::new(2.0) / F::new(3.0) * t85533 - F::new(8.0) / F::new(9.0) * t85536 + F::new(8.0) * t85540;
    (t85540, t85542)
}
