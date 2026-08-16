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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk978;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk979;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk980;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk981;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk982;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk983;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk984;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta148(t3356: f64, t3413: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3408: f64, t3410: f64, t3415: f64, t3419: f64, t3422: f64, t3425: f64, t1188: f64, t1178: f64, t439: f64, t447: f64, t3497: f64, t1161: f64, t1170: f64, t1180: f64, t1189: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3444: f64, t3447: f64, t3452: f64, t3454: f64, t3472: f64, t3477: f64, t3480: f64, t3489: f64, t3491: f64, t3496: f64, t3498: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3503, t3510, t3515) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk978(t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
        let t3516 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk979(t1188, t3515);
        let t3519 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk980(t1178);
        let t3520 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk981(t3519);
        let t3521 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk982(t3520, t439);
        let (t3522, t3523) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk983(t447);
        let t3524 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk984(t3497, t3523);
        let t3527 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk985(t1161, t1170, t1180, t1189, t3378, t3381, t3388, t3430, t3438, t3444, t3447, t3452, t3454, t3472, t3477, t3480, t3489, t3491, t3496, t3498, t3516, t3521, t3524, t435);
    (t3503, t3510, t3515, t3516, t3519, t3520, t3521, t3522, t3523, t3524, t3527)
}
