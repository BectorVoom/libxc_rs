//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta566 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1726;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1727;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1728;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1729;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1730;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta566(t43766: f64, t87145: f64, t128: f64, t43860: f64, t3362: f64, t87107: f64, t3360: f64, t22671: f64, t5046: f64, t5825: f64, t6421: f64, t1120: f64, t3367: f64, t5051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89830, t89832) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1726(t43766, t87145, t128, t43860);
        let (t89837, t89839) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1727(t3362, t87107, t128, t3360);
        let (t89841, t89843) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1728(t22671, t5046, t128, t3360);
        let (t89845, t89847) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1729(t5825, t6421, t1120, t128);
        let (t89849, t89851) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1730(t3367, t87107, t1120, t128);
        let (t89853, t89855) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1731(t22671, t5051, t1120, t128);
    (t89830, t89832, t89837, t89839, t89841, t89843, t89845, t89847, t89849, t89851, t89853, t89855)
}
