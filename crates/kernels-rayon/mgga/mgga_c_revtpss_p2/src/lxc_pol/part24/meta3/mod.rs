//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta3 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk25;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk26;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk27;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk28;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk29;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk30;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk31;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk32;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk33;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta3(t52: f64, t51: f64, sigma2: f64, t36: f64, sigma0: f64, sigma1: f64, t3: f64, t16: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t53 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk25(t52);
        let (t55, t56) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk26(t51, t53, sigma2);
        let t57 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk27(t36);
        let (t58, t59, t60) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk28(t57);
        let t61 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk29(t58, t60);
        let t64 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk30(sigma0, sigma1, sigma2);
        let t65 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk31(t3);
        let t66 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk32(t65);
        let t68 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk33(t16, t66);
    (t53, t55, t56, t57, t58, t59, t60, t61, t64, t65, t66, t68)
}
