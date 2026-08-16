//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta768 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2850;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2851;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2852;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta768(t43813: f64, t241: f64, t281: f64, t414: f64, t39484: f64, t403: f64, t409: f64, t13099: f64, t159: f64, t2435: f64, t3364: f64, t1123: f64, t9292: f64, t3373: f64, t3369: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43814, t43816, t43817, t43821, t43860, t43865) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2850(t43813, t241, t281, t414, t39484, t403, t409, t13099, t159, t2435, t3364);
        let (t43881, t43888) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2851(t43813, t1123, t9292);
        let t43890 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2852(t2435, t3373);
        let t43892 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2853(t2435, t3369);
    (t43814, t43816, t43817, t43821, t43860, t43865, t43881, t43888, t43890, t43892)
}
