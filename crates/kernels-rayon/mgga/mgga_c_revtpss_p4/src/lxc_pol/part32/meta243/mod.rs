//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1022;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1023;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta243(t1774: f64, t1211: f64, t1828: f64, t1277: f64, t3579: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t1477: f64, t476: f64, t52: f64, t475: f64, t467: f64, t1785: f64, t1803: f64, t225: f64, t6564: f64, t480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6573 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1022(t1774);
        let (t6574, t6580, t6587) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1023(t1211, t6573, t1774, t1828, t1277, t3579, t5044, t6423, t6427, t6431);
        let (t6588, t6593, t6594, t6595, t6598, t6601, t6602) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1024(t1211, t6587, t1477, t476, t52, t475, t467, t1785, t1803, t225, t6564, t480);
    (t6573, t6574, t6580, t6587, t6588, t6593, t6594, t6595, t6598, t6601, t6602)
}
