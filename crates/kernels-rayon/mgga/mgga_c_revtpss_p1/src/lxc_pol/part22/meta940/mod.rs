//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta940 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta940(t16900: f64, t698: f64, t2439: f64, t5095: f64, t16903: f64, t16907: f64, t16886: f64, t16889: f64, t5098: f64, t1179: f64, t16831: f64, t1744: f64, t3477: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58162, t58165, t58186, t58207, t58209, t58211, t58225, t58234, t58237) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3175(t16900, t698, t2439, t5095, t16903, t16907, t16886, t16889, t5098, t1179, t16831, t1744, t3477);
    (t58162, t58165, t58186, t58207, t58209, t58211, t58225, t58234, t58237)
}
