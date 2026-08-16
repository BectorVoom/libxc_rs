//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1656;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1657;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1658;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1659;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta555(t2852: f64, t87107: f64, t128: f64, t2850: f64, t22671: f64, t4573: f64, t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t291: f64, t88100: f64, t141: f64, t41294: f64, t88102: f64, t88106: f64, t930: f64, t11341: f64, t88112: f64, t2908: f64, t88120: f64, t41246: f64, t77499: f64, t77663: f64, t77667: f64, t88089: f64, t88097: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88128, t88130) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1656(t2852, t87107, t128, t2850);
        let (t88132, t88134) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1657(t22671, t4573, t128, t2850);
        let t88137 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1658(t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t88140, t88144, t88147, t88150, t88161) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1659(t291, t88100, t88137, t141, t41294, t88102, t88106, t930, t11341, t88112, t2908, t88120);
        let (t88164, t88166) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1660(t141, t2908, t88128, t41246, t77499, t77505, t77507, t77509, t77663, t77667, t88089, t88097, t88144, t88147, t88150, t88161);
    (t88128, t88130, t88132, t88134, t88140, t88144, t88147, t88150, t88161, t88164, t88166)
}
