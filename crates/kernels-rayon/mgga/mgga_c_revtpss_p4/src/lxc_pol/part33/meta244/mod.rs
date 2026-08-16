//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1088;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta244(t1477: f64, t476: f64, t52: f64, t475: f64, t467: f64, t1785: f64, t1803: f64, t225: f64, t6564: f64, t480: f64, t482: f64, t6573: f64, t371: f64, t372: f64, t1715: f64, t5277: f64, t1042: f64, t6435: f64, t6437: f64, t6441: f64, t6473: f64, t6476: f64, t6542: f64, t6544: f64, t6546: f64, t6550: f64, t6554: f64, t6558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6593, t6594, t6595, t6598, t6601, t6602, t6609) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1088(t1477, t476, t52, t475, t467, t1785, t1803, t225, t6564, t480, t482, t6573);
        let (t6611, t6618, t6619, t6622) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1089(t371, t372, t6609, t1715, t5277, t1042, t6435, t6437, t6441, t6473, t6476, t6542, t6544, t6546, t6550, t6554, t6558);
    (t6593, t6594, t6595, t6598, t6601, t6602, t6609, t6611, t6618, t6619, t6622)
}
