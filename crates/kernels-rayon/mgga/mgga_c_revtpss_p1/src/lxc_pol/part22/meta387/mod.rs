//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta387(t13665: f64, t2630: f64, t1857: f64, t3860: f64, t3863: f64, t13581: f64, t189: f64, t512: f64, t1907: f64, t9593: f64, t5566: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13666, t13667, t13668, t13669, t13671, t13672, t13673, t13674, t13680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1954(t13665, t2630, t1857, t3860, t3863, t13581, t189, t512, t1907, t9593, t5566, t749);
    (t13666, t13667, t13668, t13669, t13671, t13672, t13673, t13674, t13680)
}
