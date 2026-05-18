//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 148/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk148<F: Float>(t370: F, t432: F, t27: F, t89: F, t354: F, t366: F, t348: F) -> (F, F, F, F) {
    let t433 = t370 * t432;
    let t435 = t89 * t27 * t433;
    let t437 = -t354 - t366 / F::new(18.0) - t435 / F::new(6.0);
    let t438 = t348 * t437;
    (t433, t435, t437, t438)
}
