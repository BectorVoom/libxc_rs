//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta654(t222: f64, t40735: f64, t124: f64, t138: f64, t40649: f64, t9645: f64, t810: f64, t240: f64, t9731: f64, t10760: f64, t2664: f64, t10293: f64, t212: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t40737, t40757, t40759, t40763, t40765, t40769) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2382(t222, t40735, t124, t138, t40649, t9645, t810, t240, t9731, t10760, t2664, t10293, t212, t800);
    (t40737, t40757, t40759, t40763, t40765, t40769)
}
