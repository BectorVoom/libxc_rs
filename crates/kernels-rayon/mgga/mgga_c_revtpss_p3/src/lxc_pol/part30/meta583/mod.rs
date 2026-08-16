//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2037;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta583(t7284: f64, t94600: f64, t25884: f64, t686: f64, t72: f64, t25895: f64, t7243: f64, t9292: f64, t1032: f64, t4066: f64, t1955: f64, t25878: f64, t2453: f64, t3908: f64, t7275: f64, t1399: f64, t2434: f64, t25880: f64, t25899: f64, t3924: f64, t676: f64, t2022: f64, t9646: f64, t9648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94602, t94605, t94608, t94609, t94610, t94613) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2037(t7284, t94600, t25884, t686, t72, t25895, t7243, t9292, t1032, t4066, t1955, t25878);
        let (t94616, t94634, t94635, t94640, t94641, t94648) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2038(t2453, t3908, t7275, t1399, t2434, t25880, t25899, t3924, t676, t2022, t9646, t9648);
    (t94602, t94605, t94608, t94609, t94610, t94613, t94616, t94634, t94635, t94640, t94641, t94648)
}
