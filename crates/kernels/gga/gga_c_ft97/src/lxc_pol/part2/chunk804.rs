//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 804/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk804<F: Float>(t1882: F, t3485: F, t3408: F, t558: F, t167: F, t2185: F, t609: F, t574: F, t605: F, t3450: F, t616: F, t2142: F, t3455: F) -> (F, F, F, F, F) {
    let t12644 = F::new(4.0) / F::new(9.0) * t1882 * t3485;
    let t12645 = t3408 * t558;
    let t12647 = t2185 * t167 * t12645;
    let t12650 = t3408 * t609;
    let t12652 = t574 * t605 * t12650;
    let t12656 = t2185 * t616 * t3450;
    let t12660 = t574 * t2142 * t3455;
    (t12644, t12647, t12652, t12656, t12660)
}
