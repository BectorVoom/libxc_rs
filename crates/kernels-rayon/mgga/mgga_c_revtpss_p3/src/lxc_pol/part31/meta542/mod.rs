//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta542(t2014: f64, t29583: f64, t2034: f64, t22483: f64, t30: f64, t5966: f64, t1963: f64, t1544: f64, t1583: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t29585, t29589, t29590, t29591, t29592, t29598) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1927(t2014, t29583, t2034, t22483, t30, t5966, t1963, t1544, t1583);
    (t29585, t29589, t29590, t29591, t29592, t29598)
}
