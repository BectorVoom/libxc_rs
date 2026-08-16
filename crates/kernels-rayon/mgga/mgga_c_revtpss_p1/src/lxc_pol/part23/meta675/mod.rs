//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta675(t43813: f64, t241: f64, t281: f64, t414: f64, t39484: f64, t403: f64, t409: f64, t13099: f64, t159: f64, t1123: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t43814, t43816, t43817, t43821, t43860, t43881, t43888) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2412(t43813, t241, t281, t414, t39484, t403, t409, t13099, t159, t1123, t9292);
    (t43814, t43816, t43817, t43821, t43860, t43881, t43888)
}
