//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 788/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk788<F: Float>(t11925: F, t3128: F, t11869: F, t1113: F, t13140: F, t905: F, t11846: F, t11852: F, t11857: F, t11864: F, t13456: F, t13457: F, t13459: F, t13465: F, t13470: F, t13475: F, t902: F) -> (F, F, F, F) {
    let t13478 = 3.0 / 16.0 * t3128 * t11925;
    let t13479 = 7.0 / 96.0 * t11869;
    let t13480 = t1113 * t13140;
    let t13481 = t905 * t13480;
    let t13484 = t13456 - t13457 - t13459 - 7.0 / 256.0 * t11846 - t13465 + 7.0 / 192.0 * t11852 + t13470 - 7.0 / 96.0 * t11857 - t13475 - 7.0 / 384.0 * t11864 + t13478 + t13479 + t902 * t13481 / 1536.0;
    (t13478, t13479, t13481, t13484)
}
