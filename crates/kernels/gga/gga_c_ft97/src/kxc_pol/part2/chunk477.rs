//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 477/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk477<F: Float>(t1642: F, t2984: F, t92: F, t2993: F, t378: F, t12: F, t2998: F) -> (F, F, F, F, F) {
    let t3044 = t1642 * t2984;
    let t3045 = t92 * t3044;
    let t3047 = t378 * t2993;
    let t3048 = t92 * t3047;
    let t3050 = t12 * t2998;
    (t3044, t3045, t3047, t3048, t3050)
}
