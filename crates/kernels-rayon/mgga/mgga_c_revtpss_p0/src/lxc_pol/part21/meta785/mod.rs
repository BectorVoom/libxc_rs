//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta785 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2827;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2828;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2829;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2830;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2831;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta785(t10326: f64, t4573: f64, t128: f64, t2850: f64, t10356: f64, t1469: f64, t41296: f64, t41339: f64, t13312: f64, t2857: f64, t606: f64, t904: f64, t15144: f64, t2258: f64, t4578: f64, t15139: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51851, t51853) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2827(t10326, t4573, t128, t2850);
        let (t51856, t51858) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2828(t10356, t1469, t41296, t128, t41339);
        let (t51861, t51863) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2829(t13312, t2857, t606, t128, t904);
        let (t51865, t51867) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2830(t15144, t2258, t128, t904);
        let (t51869, t51871) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2831(t10326, t4578, t128, t904);
        let (t51873, t51875) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2832(t10356, t15139, t128, t2850);
    (t51851, t51853, t51856, t51858, t51861, t51863, t51865, t51867, t51869, t51871, t51873, t51875)
}
