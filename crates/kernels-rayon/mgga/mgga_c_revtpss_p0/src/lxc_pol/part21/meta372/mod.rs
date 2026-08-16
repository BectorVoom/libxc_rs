//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta372 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1763;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1764;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1765;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1766;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1767;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta372(t12228: f64, t3435: f64, t12248: f64, t3418: f64, t698: f64, t240: f64, t3698: f64, t3361: f64, t635: f64, t10356: f64, t141: f64, t1146: f64, t2439: f64, t3424: f64, t3421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12249, t12251, t12252) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1763(t12228, t3435, t12248, t3418, t698);
        let (t12254, t12256) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1764(t240, t3698, t3361, t635);
        let t12257 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1765(t10356, t12256);
        let (t12258, t12259, t12261) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1766(t12254, t12257, t141, t1146, t2439);
        let t12263 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1767(t3424, t698);
        let t12265 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1768(t3421, t698);
    (t12249, t12251, t12252, t12254, t12256, t12257, t12258, t12259, t12261, t12263, t12265)
}
