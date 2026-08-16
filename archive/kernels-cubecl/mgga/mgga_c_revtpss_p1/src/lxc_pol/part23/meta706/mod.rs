//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2458;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta706<F: Float>(t39494: F, t3964: F, t4096: F, t40270: F, t4089: F, t3911: F, t9692: F, t123: F, t1444: F, t3915: F, t9291: F, t2453: F, t9679: F, t138: F, t9302: F, t9674: F, t10162: F, t9303: F, t3903: F, t9292: F, t3906: F, t3907: F, t10115: F, t1421: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47454, t47455, t47474, t47478, t47480) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2458::<F>(t39494, t3964, t4096, t40270, t4089, t3911, t9692, t123, t1444, t3915, t9291, t2453, t9679);
        let (t47487, t47495, t47497, t47504, t47512) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2459::<F>(t138, t1444, t9302, t9674, t10162, t9303, t3903, t9292, t3906, t3907, t39494, t10115, t1421);
    (t47454, t47455, t47474, t47478, t47480, t47487, t47495, t47497, t47504, t47512)
}
