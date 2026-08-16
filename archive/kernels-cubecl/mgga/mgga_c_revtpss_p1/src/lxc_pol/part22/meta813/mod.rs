//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta813 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2918;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta813<F: Float>(t3911: F, t9692: F, t123: F, t1444: F, t3915: F, t9291: F, t2453: F, t9679: F, t138: F, t2438: F, t4077: F, t9302: F, t9674: F, t10162: F, t9303: F, t3903: F, t9292: F, t1445: F, t2439: F, t9640: F, t3906: F, t3907: F, t39494: F, t1426: F, t4067: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47474, t47478, t47480, t47483, t47487) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2918::<F>(t3911, t9692, t123, t1444, t3915, t9291, t2453, t9679, t138, t2438, t4077, t9302, t9674);
        let (t47495, t47497, t47500, t47504, t47506) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2919::<F>(t10162, t9303, t3903, t9292, t1445, t2439, t9640, t3906, t3907, t39494, t1426, t4067, t786);
    (t47474, t47478, t47480, t47483, t47487, t47495, t47497, t47500, t47504, t47506)
}
