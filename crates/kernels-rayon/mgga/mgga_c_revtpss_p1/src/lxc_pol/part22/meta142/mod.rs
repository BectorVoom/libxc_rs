//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk951;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk952;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk953;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk954;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta142(t1149: f64, t1150: f64, t3384: f64, t406: f64, t409: f64, t1134: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t1132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3385 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk951(t1149);
        let (t3386, t3388, t3390) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk952(t1150, t3385, t3384, t406, t409);
        let t3391 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk953(t1134);
        let (t3392, t3394, t3399) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk954(t3390, t3391, t3356, t3358, t3365, t3370, t3374);
        let (t3400, t3402, t3407) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk955(t1132, t3399, t3356, t406);
    (t3385, t3386, t3388, t3390, t3391, t3392, t3394, t3399, t3400, t3402, t3407)
}
