//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta872 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3033;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta872(t2475: f64, t808: f64, t14787: f64, t50768: f64, t14476: f64, t689: f64, t887: f64, t11028: f64, t1580: f64, t2439: f64, t10504: f64, t15002: f64, t9285: f64, t10505: f64, t137: f64, t41011: f64, t11015: f64, t4325: f64, t4477: f64, t9292: f64, t14472: f64, t14979: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51176, t51178, t51196, t51199, t51203) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3033(t2475, t808, t14787, t50768, t14476, t689, t887, t11028, t1580, t2439, t10504, t15002, t9285);
        let (t51207, t51211, t51213, t51216, t51227) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3034(t10505, t137, t15002, t41011, t11015, t4325, t4477, t9292, t14472, t2439, t887, t14979, t689, t779);
    (t51176, t51178, t51196, t51199, t51203, t51207, t51211, t51213, t51216, t51227)
}
