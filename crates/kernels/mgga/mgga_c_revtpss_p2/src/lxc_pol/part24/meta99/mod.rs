//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk571;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk572;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk573;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk574;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk575;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk576;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk577;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta99<F: Float>(t1086: F, t378: F, t994: F, t3140: F, t3143: F, t342: F, t335: F, t368: F, t3153: F, t3154: F, t1035: F, t357: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t3286 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk571::<F>(t1086, t378);
        let (t3287, t3298) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk572::<F>(t3286, t994, t3140, t3143);
        let t3299 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk573::<F>(t3298, t342);
        let t3302 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk574::<F>(t335, t368);
        let (t3303, t3304) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk575::<F>(t3153, t3302, t3154);
        let t3316 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk576::<F>(t1035, t3140);
        let t3317 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk577::<F>(t3316, t342);
        let t3318 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk578::<F>(t3303, t357);
    (t3286, t3287, t3298, t3299, t3302, t3303, t3304, t3316, t3317, t3318)
}
