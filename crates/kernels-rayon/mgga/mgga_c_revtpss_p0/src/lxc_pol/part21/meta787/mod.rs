//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta787 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta787(t141: f64, t2908: f64, t51905: f64, t15183: f64, t698: f64, t15172: f64, t2439: f64, t4625: f64, t4622: f64, t15186: f64, t51890: f64, t51892: f64, t51894: f64, t51896: f64, t51899: f64, t51902: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t51907, t51909, t51911, t51913, t51915, t51917, t51919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2835(t141, t2908, t51905, t15183, t698, t15172, t2439, t4625, t4622, t15186, t51890, t51892, t51894, t51896, t51899, t51902);
    (t51907, t51909, t51911, t51913, t51915, t51917, t51919)
}
