//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 770/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk770<F: Float>(t6472: F, t825: F, t6800: F, t2239: F, t2246: F, t2409: F, t6449: F, t831: F, t329: F, t332: F, t931: F, t376: F, t6385: F, t338: F, t353: F, t2352: F, t938: F) -> (F, F, F, F, F, F, F, F) {
    let t6801 = t6472 * t825;
    let t6802 = t6800 * t6801;
    let t6805 = t2246 * t2239;
    let t6810 = t2409 * t831 * t6449;
    let t6816 = t329 * t332 * t931;
    let t6817 = t376 * t6385;
    let t6819 = t338 * t353 * t6817;
    let t6822 = t2352 * t938;
    (t6801, t6802, t6805, t6810, t6816, t6817, t6819, t6822)
}
