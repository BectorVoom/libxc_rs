//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta912 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3117;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta912(t3133: f64, t3155: f64, t1062: f64, t43154: f64, t11940: f64, t3105: f64, t11923: f64, t15926: f64, t11922: f64, t16016: f64, t4899: f64, t11994: f64, t15734: f64, t15830: f64, t3111: f64, t11866: f64, t16035: f64, t16088: f64, t342: f64, t380: f64, t16219: f64, t3241: f64, t12047: f64, t53552: f64, t15810: f64, t3127: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54950, t54982, t54988, t54991, t54994, t55000) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3117(t3133, t3155, t1062, t43154, t11940, t3105, t11923, t15926, t11922, t16016, t4899, t11994, t15734);
        let (t55002, t55004, t55011, t55033, t55046, t55058) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3118(t15830, t3111, t11866, t16035, t16088, t342, t380, t16219, t3241, t12047, t53552, t15810, t3127, t3172);
    (t54950, t54982, t54988, t54991, t54994, t55000, t55002, t55004, t55011, t55033, t55046, t55058)
}
