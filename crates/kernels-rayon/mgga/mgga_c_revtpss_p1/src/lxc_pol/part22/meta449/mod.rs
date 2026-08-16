//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2099;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2100;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2101;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2102;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2103;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2104;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2105;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta449(t15127: f64, t2852: f64, t4186: f64, t606: f64, t2850: f64, t128: f64, t2258: f64, t4573: f64, t11144: f64, t1469: f64, t2251: f64, t11142: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15128, t15130) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2099(t15127, t2852, t4186, t606);
        let (t15131, t15132) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2100(t15130, t2850, t128);
        let t15135 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2101(t2258, t4573);
        let (t15136, t15137) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2102(t15135, t2850, t128);
        let (t15139, t15140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2103(t11144, t1469, t2251);
        let (t15141, t15142) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2104(t11142, t15140, t128);
        let t15145 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2105(t2857, t4186, t606);
    (t15128, t15130, t15131, t15132, t15135, t15136, t15137, t15139, t15140, t15141, t15142, t15145)
}
