//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta699(t576: f64, t588: f64, t15: f64, t27: f64, t11: f64, t22: f64, t10276: f64, t584: f64, t596: f64, t20: f64, t2237: f64, t12: f64, t14: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t45928, t45931, t45933, t45934, t45938, t45941, t45944) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2521(t576, t588, t15, t27, t11, t22, t10276, t584, t596, t20, t2237, t12, t14);
    (t45928, t45931, t45933, t45934, t45938, t45941, t45944)
}
