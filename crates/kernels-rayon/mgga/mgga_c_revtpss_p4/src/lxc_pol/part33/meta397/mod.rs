//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta397(t17350: f64, t3782: f64, t1263: f64, t1794: f64, t372: f64, t11262: f64, t1796: f64, t1247: f64, t12915: f64, t247: f64, t5230: f64, t5384: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t17351, t17353, t17361, t17362, t17373, t17375) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1447(t17350, t3782, t1263, t1794, t372, t11262, t1796, t1247, t12915, t247, t5230, t5384);
    (t17351, t17353, t17361, t17362, t17373, t17375)
}
