//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta813 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2918;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta813(t3911: f64, t9692: f64, t123: f64, t1444: f64, t3915: f64, t9291: f64, t2453: f64, t9679: f64, t138: f64, t2438: f64, t4077: f64, t9302: f64, t9674: f64, t10162: f64, t9303: f64, t3903: f64, t9292: f64, t1445: f64, t2439: f64, t9640: f64, t3906: f64, t3907: f64, t39494: f64, t1426: f64, t4067: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47474, t47478, t47480, t47483, t47487) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2918(t3911, t9692, t123, t1444, t3915, t9291, t2453, t9679, t138, t2438, t4077, t9302, t9674);
        let (t47495, t47497, t47500, t47504, t47506) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2919(t10162, t9303, t3903, t9292, t1445, t2439, t9640, t3906, t3907, t39494, t1426, t4067, t786);
    (t47474, t47478, t47480, t47483, t47487, t47495, t47497, t47500, t47504, t47506)
}
