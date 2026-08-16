//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1589;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta423(t43880: f64, t43907: f64, t1132: f64, t2439: f64, t3418: f64, t141: f64, t3417: f64, t43869: f64, t1145: f64, t43875: f64, t43839: f64, t43852: f64, t43847: f64, t12283: f64, t698: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43908, t43909, t43911, t43914, t43917, t43920, t43923) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1589(t43880, t43907, t1132, t2439, t3418, t141, t3417, t43869, t1145, t43875, t43839, t43852);
        let (t43926, t43928, t43936) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1590(t1145, t141, t43847, t12283, t698, t43858, t43862, t43865, t43871, t43877, t43883, t43909, t43911, t43914, t43917, t43920, t43923);
    (t43908, t43909, t43911, t43914, t43917, t43920, t43923, t43926, t43928, t43936)
}
