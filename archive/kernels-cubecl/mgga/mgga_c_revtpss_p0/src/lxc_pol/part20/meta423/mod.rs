//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta423 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1589;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta423<F: Float>(t43880: F, t43907: F, t1132: F, t2439: F, t3418: F, t141: F, t3417: F, t43869: F, t1145: F, t43875: F, t43839: F, t43852: F, t43847: F, t12283: F, t698: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43883: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t43908, t43909, t43911, t43914, t43917, t43920, t43923) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1589::<F>(t43880, t43907, t1132, t2439, t3418, t141, t3417, t43869, t1145, t43875, t43839, t43852);
        let (t43926, t43928, t43936) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1590::<F>(t1145, t141, t43847, t12283, t698, t43858, t43862, t43865, t43871, t43877, t43883, t43909, t43911, t43914, t43917, t43920, t43923);
    (t43908, t43909, t43911, t43914, t43917, t43920, t43923, t43926, t43928, t43936)
}
