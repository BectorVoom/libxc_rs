//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 640/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk640<F: Float>(t7778: F, t7782: F, t7820: F, t8195: F, t7771: F, t8189: F, t1851: F, t480: F, t1827: F, t1882: F, t494: F, t8232: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8444 = t7778 / F::cast_from(9.0_f64);
    let t8446 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7782;
    let t8449 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t7820;
    let t8452 = t8195 / F::cast_from(3.0_f64);
    let t8454 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7771;
    let t8455 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t8189;
    let t8466 = t480 * t1851;
    let t8471 = t1882 * t1827;
    let t8475 = t8232 * t494;
    (t8444, t8446, t8449, t8452, t8454, t8455, t8466, t8471, t8475)
}
