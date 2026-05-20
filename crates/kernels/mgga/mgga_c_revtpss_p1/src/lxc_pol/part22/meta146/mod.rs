//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk967;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk968;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk969;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk970;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta146<F: Float>(t1169: F, t3453: F, t3356: F, t3413: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F, t1159: F, t426: F, t434: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3454, t3459, t3466, t3471) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk967::<F>(t1169, t3453, t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
        let (t3472, t3475) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk968::<F>(t1169, t3471, t1159);
        let t3476 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk969::<F>(t3475);
        let t3477 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk970::<F>(t3476, t426);
        let (t3478, t3479) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk971::<F>(t434);
    (t3454, t3459, t3466, t3471, t3472, t3475, t3476, t3477, t3478, t3479)
}
