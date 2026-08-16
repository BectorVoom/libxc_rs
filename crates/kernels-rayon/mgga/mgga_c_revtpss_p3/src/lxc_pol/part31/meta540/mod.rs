//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta540(t29543: f64, t77: f64, t5872: f64, t84: f64, t5819: f64, t603: f64, t5826: f64, t5816: f64, t1923: f64, t1928: f64, t25157: f64, t28127: f64, t28138: f64, t28151: f64, t28154: f64, t29513: f64, t29526: f64, t29529: f64, t29533: f64, t29538: f64, t6958: f64, t7702: f64, t7706: f64, t7709: f64, t7716: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29567) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1925(t29543, t77, t5872, t84, t5819, t603, t5826, t5816, t1923, t1928, t25157, t28127, t28138, t28151, t28154, t29513, t29526, t29529, t29533, t29538, t6958, t7702, t7706, t7709, t7716, t7720);
    (t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29567)
}
