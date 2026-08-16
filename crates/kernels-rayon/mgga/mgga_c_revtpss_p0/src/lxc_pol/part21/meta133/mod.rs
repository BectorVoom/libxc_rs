//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk857;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk858;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk859;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk860;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk861;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta133(t3147: f64, t365: f64, t3144: f64, t3141: f64, t1043: f64, t373: f64, t73: f64, t357: f64, t1042: f64, t1036: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3148, t3149, t3150, t3151) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk857(t3147, t365, t3144, t3141, t1043);
        let (t3152, t3153) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk858(t3151, t373, t73);
        let t3154 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk859(t357);
        let t3155 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk860(t3153, t3154);
        let (t3156, t3157) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk861(t3152, t3155, t1042);
        let (t3160, t3161, t3162) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk862(t1036, t3148, t3141, t3153, t357);
    (t3149, t3150, t3151, t3152, t3153, t3154, t3155, t3156, t3157, t3160, t3161, t3162)
}
