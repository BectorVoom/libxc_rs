//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 856/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk856<F: Float>(t13468: F, t3138: F, t13220: F, t6659: F, t858: F, t884: F, t11925: F, t3128: F, t11869: F, t1113: F, t13140: F, t905: F) -> (F, F, F, F, F, F) {
    let t13470 = t3138 * t13468 / F::new(16.0);
    let t13473 = t6659 * t858 * t13220;
    let t13475 = t884 * t13473 / F::new(4.0);
    let t13478 = F::new(3.0) / F::new(16.0) * t3128 * t11925;
    let t13479 = F::new(7.0) / F::new(96.0) * t11869;
    let t13480 = t1113 * t13140;
    let t13481 = t905 * t13480;
    (t13470, t13473, t13475, t13478, t13479, t13481)
}
