//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 812/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk812<F: Float>(t12754: F, t574: F, t605: F, t12600: F, t144: F, t1060: F, t1651: F, t569: F, t1643: F, t2205: F, t2230: F, t925: F) -> (F, F, F, F, F) {
    let t12756 = t574 * t605 * t12754;
    let t12759 = t144 * t12600;
    let t12763 = t569 * t1060 * t1651;
    let t12767 = t2205 * t1060 * t1643;
    let t12771 = t569 * t2230 * t925;
    (t12756, t12759, t12763, t12767, t12771)
}
