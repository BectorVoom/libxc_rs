//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 736/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk736<F: Float>(t3837: F, t6501: F, t3765: F, t6402: F, t3816: F, t6627: F, t2319: F, t3810: F, t3792: F, t6183: F, t3116: F, t2164: F, t3880: F) -> (F, F, F, F, F, F, F) {
    let t11846 = t6501 * t3837;
    let t11852 = t6402 * t3765;
    let t11857 = t6627 * t3816;
    let t11864 = t2319 * t3810;
    let t11868 = t6183 * t3792;
    let t11869 = t3116 * t11868;
    let t11912 = t2164 * t3880;
    (t11846, t11852, t11857, t11864, t11868, t11869, t11912)
}
