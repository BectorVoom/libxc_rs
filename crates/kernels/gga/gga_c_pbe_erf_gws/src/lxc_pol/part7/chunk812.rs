//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 812/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk812<F: Float>(t6237: F, t6313: F, t6380: F, t6462: F, t6516: F, t6584: F, t6655: F, t6720: F, t898: F, t338: F, t353: F, t329: F, t6594: F) -> (F, F, F, F) {
    let t6723 = t6237 + t6313 + t6380 + t6462 + t6516 + t6584 + t6655 + t6720;
    let t6724 = t898 * t6723;
    let t6726 = t338 * t353 * t6724;
    let t6729 = t329 * t6594;
    (t6723, t6724, t6726, t6729)
}
