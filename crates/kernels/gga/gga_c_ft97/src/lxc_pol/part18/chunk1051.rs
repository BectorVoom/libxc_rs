//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1051/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1051<F: Float>(t19: F, t7: F, t6: F, t64: F, t8052: F, t66: F, t7918: F, t5537: F, t76: F, t8050: F, t378: F, t7241: F, t1586: F, t1642: F, t422: F, t626: F) -> (F, F, F, F, F, F, F, F) {
    let t37991 = t7 * t19;
    let t38013 = t64 * t8052 * t6;
    let t38149 = t7918 * t66;
    let t38150 = t38149 * t5537;
    let t38241 = 1.0 / t8050 / t76;
    let t38262 = t378 * t7241;
    let t38268 = t1642 * t1586;
    let t38308 = t626 * t422;
    (t37991, t38013, t38149, t38150, t38241, t38262, t38268, t38308)
}
