//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 426/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk426<F: Float>(t2372: F, t2373: F, t27: F, t89: F, t196: F, t122: F) -> (F, F, F, F) {
    let t2374 = t2372 * t2373;
    let t2376 = t89 * t27 * t2374;
    let t2378 = F::cast_from(1.0_f64) / t196;
    let t2379 = t122 * t2378;
    (t2374, t2376, t2378, t2379)
}
