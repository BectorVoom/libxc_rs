//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 391/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk391<F: Float>(t408: F, t938: F, t64: F, t66: F, t1710: F, t173: F, t934: F, t419: F, t1736: F, t420: F) -> (F, F, F, F, F) {
    let t3070 = t408 * t938;
    let t3076 = t64 * t66;
    let t3077 = t1710 * t938;
    let t3085 = t173 * t934;
    let t3086 = t419 * t3085;
    let t3088 = t420 * t1736;
    (t3070, t3076, t3077, t3086, t3088)
}
