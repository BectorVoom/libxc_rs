//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1401;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta443(t39494: f64, t3964: f64, t4096: f64, t2453: f64, t9679: f64, t3906: f64, t3907: f64, t1359: f64, t39501: f64, t10115: f64, t555: f64, t123: f64, t125: f64, t1358: f64, t8779: f64, t9645: f64, t268: f64, t39644: f64, t556: f64, t561: f64, t786: f64, t9656: f64, t4146: f64, t1892: f64, t9646: f64, t9648: f64, t1904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47454, t47480, t47504, t47561, t47567, t47591) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1401(t39494, t3964, t4096, t2453, t9679, t3906, t3907, t1359, t39501, t10115, t555, t123, t125, t1358, t8779, t9645);
        let (t47601, t47603, t47672, t47764, t47772) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1402(t268, t39644, t556, t561, t8779, t786, t9656, t4146, t1892, t9646, t9648, t1904, t47567);
    (t47454, t47480, t47504, t47561, t47591, t47601, t47603, t47672, t47764, t47772)
}
