//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta151 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1000;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1001;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1002;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1003;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1004;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1005;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1006;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta151(t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t1211: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3528: f64, t3530: f64, t3533: f64, t3537: f64, t3541: f64, t3545: f64, t1250: f64, t482: f64, t1042: f64, t3140: f64, t460: f64, t1242: f64, t472: f64, t474: f64, t3147: f64, t479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3579, t3584) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1000(t3356, t3358, t3365, t3370, t3374);
        let t3585 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1001(t1211, t3584);
        let t3588 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1002(t3378, t3381, t3388, t3430, t3438, t3528, t3530, t3533, t3537, t3541, t3545);
        let (t3590, t3591) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1003(t1250, t3588, t482, t1042);
        let t3594 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1004(t3140, t460);
        let t3596 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1005(t1242, t472);
        let (t3597, t3598, t3599) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1006(t3596, t474, t3147, t479);
        let t3600 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1007(t3594, t3599);
    (t3579, t3584, t3585, t3588, t3590, t3591, t3594, t3596, t3597, t3598, t3599, t3600)
}
