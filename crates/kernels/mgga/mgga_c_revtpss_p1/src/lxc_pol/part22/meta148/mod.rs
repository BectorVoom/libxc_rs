//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta148 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk978;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk979;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk980;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk981;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk982;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk983;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk984;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta148<F: Float>(t3356: F, t3413: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F, t1188: F, t1178: F, t439: F, t447: F, t3497: F, t1161: F, t1170: F, t1180: F, t1189: F, t3378: F, t3381: F, t3388: F, t3430: F, t3438: F, t3444: F, t3447: F, t3452: F, t3454: F, t3472: F, t3477: F, t3480: F, t3489: F, t3491: F, t3496: F, t3498: F, t435: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3503, t3510, t3515) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk978::<F>(t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
        let t3516 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk979::<F>(t1188, t3515);
        let t3519 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk980::<F>(t1178);
        let t3520 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk981::<F>(t3519);
        let t3521 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk982::<F>(t3520, t439);
        let (t3522, t3523) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk983::<F>(t447);
        let t3524 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk984::<F>(t3497, t3523);
        let t3527 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk985::<F>(t1161, t1170, t1180, t1189, t3378, t3381, t3388, t3430, t3438, t3444, t3447, t3452, t3454, t3472, t3477, t3480, t3489, t3491, t3496, t3498, t3516, t3521, t3524, t435);
    (t3503, t3510, t3515, t3516, t3519, t3520, t3521, t3522, t3523, t3524, t3527)
}
