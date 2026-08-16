//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta395(t3678: f64, t5327: f64, t5323: f64, t3667: f64, t5362: f64, t1789: f64, t371: f64, t676: f64, t1235: f64, t1769: f64, t3565: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17296, t17298, t17301, t17303, t17304, t17306, t17307) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1445(t3678, t5327, t5323, t3667, t5362, t1789, t371, t676, t1235, t1769, t3565, t225);
    (t17296, t17298, t17301, t17303, t17304, t17306, t17307)
}
