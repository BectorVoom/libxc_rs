//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta591(t94385: f64, t9675: f64, t94589: f64, t7289: f64, t94377: f64, t7285: f64, t9288: f64, t7284: f64, t7243: f64, t9292: f64, t2453: f64, t3908: f64, t7275: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t94590, t94591, t94593, t94600, t94602, t94608, t94616) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2016(t94385, t9675, t94589, t7289, t94377, t7285, t9288, t7284, t7243, t9292, t2453, t3908, t7275);
    (t94590, t94591, t94593, t94600, t94602, t94608, t94616)
}
