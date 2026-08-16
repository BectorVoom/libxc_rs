//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta287(t4077: f64, t676: f64, t123: f64, t9680: f64, t1444: f64, t2434: f64, t3915: f64, t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9681, t9682, t9683, t9685, t9686, t9687, t9691, t9692, t9694) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1701(t4077, t676, t123, t9680, t1444, t2434, t3915, t1359, t9292, t1363, t9288, t1362);
    (t9681, t9682, t9683, t9685, t9686, t9687, t9691, t9692, t9694)
}
