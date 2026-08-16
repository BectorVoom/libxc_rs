//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta769 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta769(t2439: f64, t3418: f64, t406: f64, t12555: f64, t3515: f64, t43813: f64, t1126: f64, t12226: f64, t3382: f64, t3431: f64, t408: f64, t43816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43911, t43946, t43977, t43995, t44012, t44017, t44039, t44040) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2854(t2439, t3418, t406, t12555, t3515, t43813, t1126, t12226, t3382, t3431, t408, t43816);
    (t43911, t43946, t43977, t43995, t44012, t44017, t44039, t44040)
}
