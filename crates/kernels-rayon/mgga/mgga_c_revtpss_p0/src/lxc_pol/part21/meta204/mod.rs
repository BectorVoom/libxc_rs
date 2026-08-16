//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta204 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1232;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1233;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1234;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1235;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1236;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1237;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1238;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta204(t1015: f64, t4186: f64, t1012: f64, t3147: f64, t72: f64, t3088: f64, t3299: f64, t1668: f64, t3153: f64, t1043: f64, t3154: f64, t3117: f64, t3317: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4886, t4887, t4890) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1232(t1015, t4186, t1012, t3147, t72);
        let t4891 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1233(t3088, t4890);
        let t4892 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1234(t3299, t4891);
        let t4893 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1235(t1668, t3153);
        let t4894 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1236(t1043, t3154);
        let (t4895, t4896) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1237(t4893, t4894, t3117);
        let t4899 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1238(t3317, t4891);
        let t4900 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1239(t1043, t357);
    (t4886, t4887, t4890, t4891, t4892, t4893, t4894, t4895, t4896, t4899, t4900)
}
