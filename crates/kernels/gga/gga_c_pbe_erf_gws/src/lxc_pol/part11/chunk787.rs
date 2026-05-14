//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 787/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk787<F: Float>(t11811: F, t11817: F, t11984: F, t3793: F, t13368: F, t343: F, t858: F, t867: F, t866: F, t13431: F, t3131: F, t3139: F, t3138: F, t13220: F, t6659: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13456 = 7.0 / 96.0 * t11811;
    let t13457 = 7.0 / 96.0 * t11817;
    let t13459 = t11984 * t3793 / 32.0;
    let t13461 = t13368 * t343;
    let t13463 = t867 * t858 * t13461;
    let t13465 = t866 * t13463 / 96.0;
    let t13468 = t3139 * t3131 * t13431;
    let t13470 = t3138 * t13468 / 16.0;
    let t13473 = t6659 * t858 * t13220;
    let t13475 = t884 * t13473 / 4.0;
    (t13456, t13457, t13459, t13461, t13463, t13465, t13468, t13470, t13473, t13475)
}
