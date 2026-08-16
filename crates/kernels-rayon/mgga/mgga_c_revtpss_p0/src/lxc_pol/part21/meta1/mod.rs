//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta1 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk9;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk10;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk11;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk12;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk13;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk14;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk15;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk16;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk17;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk18;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk19;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta1(t14: f64, t11: f64, t16: f64, t12: f64, t15: f64, t17: f64, t9: f64, t5: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t19 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk9(t14);
        let t20 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk10(t11, t19);
        let t21 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk11(t16);
        let t22 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk12(t21);
        let t25 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk13(t12, t14, t19);
        let (t26, t27) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk14(t16, t21);
        let t29 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk15(t15, t17, t20, t22, t25, t27, t9);
        let t30 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk16(t5);
        let (t32, t33) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk17(t30, t5, zeta_threshold);
        let t36 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk18(t30, t33, t32, t5, zeta_threshold);
        let t37 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk19(t36);
    (t19, t20, t21, t22, t25, t26, t27, t29, t30, t33, t36, t37)
}
