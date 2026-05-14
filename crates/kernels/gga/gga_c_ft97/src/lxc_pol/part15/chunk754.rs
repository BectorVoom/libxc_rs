//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 754/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk754<F: Float>(t21949: F, t2771: F, t192: F, t22161: F, t852: F, t10613: F, t21958: F, t21602: F, t2766: F, t21196: F, t4199: F, t21969: F, t21973: F, t21588: F, t848: F, t21204: F, t4206: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22302 = t2771 * t21949;
    let t22306 = t192 * t852 * t22161;
    let t22310 = t10613 * t21958;
    let t22313 = t2766 * t21602;
    let t22316 = t4199 * t21196;
    let t22319 = t2771 * t21969;
    let t22321 = t2771 * t21973;
    let t22323 = t848 * t21588;
    let t22326 = t4206 * t21204;
    (t22302, t22306, t22310, t22313, t22316, t22319, t22321, t22323, t22326)
}
