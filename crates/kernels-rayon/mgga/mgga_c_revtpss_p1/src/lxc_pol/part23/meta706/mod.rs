//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2458;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta706(t39494: f64, t3964: f64, t4096: f64, t40270: f64, t4089: f64, t3911: f64, t9692: f64, t123: f64, t1444: f64, t3915: f64, t9291: f64, t2453: f64, t9679: f64, t138: f64, t9302: f64, t9674: f64, t10162: f64, t9303: f64, t3903: f64, t9292: f64, t3906: f64, t3907: f64, t10115: f64, t1421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47454, t47455, t47474, t47478, t47480) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2458(t39494, t3964, t4096, t40270, t4089, t3911, t9692, t123, t1444, t3915, t9291, t2453, t9679);
        let (t47487, t47495, t47497, t47504, t47512) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2459(t138, t1444, t9302, t9674, t10162, t9303, t3903, t9292, t3906, t3907, t39494, t10115, t1421);
    (t47454, t47455, t47474, t47478, t47480, t47487, t47495, t47497, t47504, t47512)
}
