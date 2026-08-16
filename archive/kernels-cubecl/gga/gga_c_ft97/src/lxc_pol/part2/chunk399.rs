//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 399/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk399<F: Float>(t2190: F, t574: F, t605: F, t1882: F, t571: F, t379: F, t569: F, t616: F, t1651: F, t167: F, t143: F, t1642: F) -> (F, F, F, F, F) {
    let t2192 = t574 * t605 * t2190;
    let t2195 = t1882 * t571;
    let t2198 = t569 * t616 * t379;
    let t2202 = t569 * t167 * t1651;
    let t2205 = t1642 * t143;
    (t2192, t2195, t2198, t2202, t2205)
}
