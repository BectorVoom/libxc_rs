//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1398/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1398<F: Float>(t1014: F, t2598: F, t2601: F, t30004: F, t30007: F, t30009: F, t30012: F, t30015: F, t30018: F, t30021: F, t30024: F, t30028: F, t30031: F, t30034: F, t30041: F) -> F {
    let t30304 = -F::new(0.34631718211362927518e2) * t1014 * t2598 * t30041 * t2601 - t30004 - t30007 + t30009 - t30012 - t30015 - t30018 - t30021 + t30024 + t30028 + t30031 + t30034;
    t30304
}
