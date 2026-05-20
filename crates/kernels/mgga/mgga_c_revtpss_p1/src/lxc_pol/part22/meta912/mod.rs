//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta912 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3117;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta912<F: Float>(t3133: F, t3155: F, t1062: F, t43154: F, t11940: F, t3105: F, t11923: F, t15926: F, t11922: F, t16016: F, t4899: F, t11994: F, t15734: F, t15830: F, t3111: F, t11866: F, t16035: F, t16088: F, t342: F, t380: F, t16219: F, t3241: F, t12047: F, t53552: F, t15810: F, t3127: F, t3172: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54950, t54982, t54988, t54991, t54994, t55000) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3117::<F>(t3133, t3155, t1062, t43154, t11940, t3105, t11923, t15926, t11922, t16016, t4899, t11994, t15734);
        let (t55002, t55004, t55011, t55033, t55046, t55058) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3118::<F>(t15830, t3111, t11866, t16035, t16088, t342, t380, t16219, t3241, t12047, t53552, t15810, t3127, t3172);
    (t54950, t54982, t54988, t54991, t54994, t55000, t55002, t55004, t55011, t55033, t55046, t55058)
}
