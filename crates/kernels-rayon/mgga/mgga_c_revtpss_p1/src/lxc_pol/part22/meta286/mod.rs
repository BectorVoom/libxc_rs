//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1699;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta286(t4131: f64, t676: f64, t123: f64, t3915: f64, t2453: f64, t3914: f64, t1444: f64, t2438: f64, t138: f64, t4075: f64, t556: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9671, t9672, t9674) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1699(t4131, t676, t123, t3915, t2453, t3914);
        let (t9675, t9676, t9677, t9679, t9680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1700(t1444, t2438, t138, t9674, t4075, t556, t786);
    (t9671, t9672, t9674, t9675, t9676, t9677, t9679, t9680)
}
