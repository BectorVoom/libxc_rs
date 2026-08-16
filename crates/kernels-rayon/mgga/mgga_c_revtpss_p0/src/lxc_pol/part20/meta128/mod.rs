//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk728;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk729;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk730;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk731;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk732;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk733;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta128(t3495: f64, t439: f64, t1187: f64, t1188: f64, t3356: f64, t3413: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3408: f64, t3410: f64, t3415: f64, t3419: f64, t3422: f64, t3425: f64, t1178: f64, t447: f64, t1161: f64, t1170: f64, t1180: f64, t1189: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3444: f64, t3447: f64, t3452: f64, t3454: f64, t3472: f64, t3477: f64, t3480: f64, t3489: f64, t3491: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3496, t3497) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk728(t3495, t439, t1187);
        let (t3498, t3515) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk729(t1188, t3497, t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
        let t3516 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk730(t1188, t3515);
        let t3519 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk731(t1178);
        let t3520 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk732(t3519);
        let (t3521, t3522, t3523) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk733(t3520, t439, t447);
        let (t3524, t3527) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk734(t3497, t3523, t1161, t1170, t1180, t1189, t3378, t3381, t3388, t3430, t3438, t3444, t3447, t3452, t3454, t3472, t3477, t3480, t3489, t3491, t3496, t3498, t3516, t3521, t435);
    (t3496, t3497, t3498, t3515, t3516, t3519, t3520, t3521, t3522, t3523, t3524, t3527)
}
