//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk904;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk905;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk906;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta135(t1025: f64, t3215: f64, t3075: f64, t373: f64, t371: f64, t372: f64, t225: f64, t3046: f64, t366: f64, t362: f64, t40: f64, t611: f64, t361: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3216, t3218, t3220, t3223) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk904(t1025, t3215, t3075, t373, t371, t372, t225, t3046);
        let t3224 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk905(t3223, t366);
        let t3229 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk906(t362, t40, t611);
        let t3230 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk907(t3229, t361);
    (t3216, t3218, t3220, t3223, t3224, t3229, t3230)
}
