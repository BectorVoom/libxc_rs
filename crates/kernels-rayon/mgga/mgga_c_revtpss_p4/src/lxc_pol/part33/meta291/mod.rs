//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1280;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta291(t138: f64, t9675: f64, t9674: f64, t4075: f64, t556: f64, t786: f64, t1444: f64, t2434: f64, t123: f64, t3915: f64, t1359: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9676, t9677, t9680, t9685, t9686, t9687, t9691) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1280(t138, t9675, t9674, t4075, t556, t786, t1444, t2434, t123, t3915, t1359, t9292);
    (t9676, t9677, t9680, t9685, t9686, t9687, t9691)
}
