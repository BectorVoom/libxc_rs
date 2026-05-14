//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 943/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk943<F: Float>(t39: F, t55: F, t59: F, t87: F, t1759: F, t1784: F, t1790: F, t1792: F, t533: F, t6446: F, t1758: F, t6452: F, t6454: F, t22124: F, t22128: F, t22130: F, t22134: F, t22136: F, t22141: F, t22143: F, t22152: F) -> (F, F, F, F, F) {
    let t22274 = 24.0 * t39 * t55 * t59 * t87;
    let t22277 = 36.0 * t1790 * t1759 * t1784;
    let t22281 = 0.64327297288604419288e2 * t1790 * t6446 * t1792 * t533;
    let t22285 = 0.3103500882342370105e4 * t6452 * t1758 * t6454 * t1784;
    let t22286 = t22124 - t22128 - t22130 - t22134 - t22136 + t22141 - t22143 + t22152 + t22274 + t22277 + t22281 + t22285;
    (t22274, t22277, t22281, t22285, t22286)
}
