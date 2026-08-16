//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta269(t10292: f64, t25: f64, t2246: f64, t599: f64, t88: f64, t89: f64, t90: f64, t29: f64, t46: f64, t47: f64, t58: f64, t59: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10293, t10295, t10301, t10308, t10309, t10355, t10368) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1479(t10292, t25, t2246, t599, t88, t89, t90, t29, t46, t47, t58, t59);
    (t10293, t10295, t10301, t10308, t10309, t10355, t10368)
}
