//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 750/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk750<F: Float>(t6619: F, t850: F, t852: F, t860: F, t2087: F, t2142: F, t899: F, t912: F, t923: F) -> (F, F, F, F) {
    let t6621 = t850 * t6619 * t852;
    let t6623 = t6621 * t860 / 96.0;
    let t6624 = t2087 * t2142;
    let t6625 = 7.0 / 96.0 * t6624;
    let t6627 = t899 * t912 * t923;
    (t6621, t6623, t6625, t6627)
}
