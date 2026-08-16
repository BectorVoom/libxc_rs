//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1438;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta252(t3896: f64, t9303: f64, t1419: f64, t785: f64, t1358: f64, t2439: f64, t784: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9639, t9640, t9641, t9642, t9644, t9645, t9646) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1438(t3896, t9303, t1419, t785, t1358, t2439, t784, t209);
    (t9639, t9640, t9641, t9642, t9644, t9645, t9646)
}
