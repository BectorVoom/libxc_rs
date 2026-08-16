//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta257(t4077: f64, t676: f64, t123: f64, t9680: f64, t1444: f64, t2434: f64, t3915: f64, t1424: f64, t4071: f64, t4132: f64, t9632: f64, t9636: f64, t9639: f64, t9642: f64, t9650: f64, t9652: f64, t9659: f64, t9666: f64, t9668: f64, t9672: f64, t9677: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9681, t9682, t9683, t9685, t9686, t9687, t9689) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1449(t4077, t676, t123, t9680, t1444, t2434, t3915, t1424, t4071, t4132, t9632, t9636, t9639, t9642, t9650, t9652, t9659, t9666, t9668, t9672, t9677);
    (t9681, t9682, t9683, t9685, t9686, t9687, t9689)
}
