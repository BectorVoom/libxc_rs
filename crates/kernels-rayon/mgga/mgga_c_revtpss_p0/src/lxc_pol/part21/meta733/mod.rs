//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta733 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2580;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta733(t10008: f64, t1432: f64, t686: f64, t72: f64, t268: f64, t39644: f64, t546: f64, t555: f64, t8779: f64, t4107: f64, t9288: f64, t10107: f64, t3964: f64, t9285: f64, t39494: f64, t4096: f64, t40270: f64, t4089: f64, t138: f64, t2438: f64, t4131: f64, t9674: f64, t1444: f64, t2782: f64, t4075: f64, t556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47436, t47442, t47444, t47450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2580(t10008, t1432, t686, t72, t268, t39644, t546, t555, t8779, t4107, t9288, t10107, t3964, t9285);
        let (t47454, t47455, t47466, t47472) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2581(t39494, t3964, t4096, t40270, t4089, t138, t2438, t4131, t9674, t1444, t2782, t4075, t556);
    (t47436, t47442, t47444, t47450, t47454, t47455, t47466, t47472)
}
