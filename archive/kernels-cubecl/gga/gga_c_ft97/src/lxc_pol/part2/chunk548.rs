//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 548/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk548<F: Float>(t3140: F, t582: F, t1037: F, t458: F, t2102: F, t3338: F, t1017: F, t2: F, t1985: F, t558: F, t24: F, t3408: F, t586: F) -> (F, F, F, F, F, F) {
    let t3510 = t582 * t3140;
    let t3513 = t458 * t1037;
    let t3515 = t2102 * t3338;
    let t3518 = t2 * t1017;
    let t3520 = t1985 * t3518 * t558;
    let t3524 = t24 * t586 * t3408;
    (t3510, t3513, t3515, t3518, t3520, t3524)
}
