//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta606(t2155: f64, t44126: f64, t2172: f64, t4153: f64, t27110: f64, t571: f64, t27833: f64, t7316: f64, t13426: f64, t7003: f64, t18227: f64, t25861: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t97498, t97580, t97586, t97604, t97606, t97608, t97610) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2069(t2155, t44126, t2172, t4153, t27110, t571, t27833, t7316, t13426, t7003, t18227, t25861, t4248);
    (t97498, t97580, t97586, t97604, t97606, t97608, t97610)
}
