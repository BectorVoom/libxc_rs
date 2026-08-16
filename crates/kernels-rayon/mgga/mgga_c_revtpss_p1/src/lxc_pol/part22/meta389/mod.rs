//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta389(t1892: f64, t785: f64, t1358: f64, t2439: f64, t1903: f64, t4075: f64, t1444: f64, t556: f64, t2782: f64, t212: f64, t5710: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13725, t13726, t13727, t13729, t13730, t13731, t13733, t13734, t13735, t13737) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1958(t1892, t785, t1358, t2439, t1903, t4075, t1444, t556, t2782, t212, t5710, t689);
    (t13725, t13726, t13727, t13729, t13730, t13731, t13733, t13734, t13735, t13737)
}
