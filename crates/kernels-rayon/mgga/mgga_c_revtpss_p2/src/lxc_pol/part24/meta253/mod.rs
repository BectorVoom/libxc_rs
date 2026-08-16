//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta253(t12078: f64, t15905: f64, t1086: f64, t4746: f64, t3090: f64, t15822: f64, t3160: f64, t1065: f64, t2852: f64, t2857: f64, t357: f64, t1014: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15906, t15925, t15926, t15932, t15935, t15962, t15987) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1020(t12078, t15905, t1086, t4746, t3090, t15822, t3160, t1065, t2852, t2857, t357, t1014, t140);
    (t15906, t15925, t15926, t15932, t15935, t15962, t15987)
}
