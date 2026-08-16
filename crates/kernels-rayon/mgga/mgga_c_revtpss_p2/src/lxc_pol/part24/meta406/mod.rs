//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1345;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta406(t220: f64, t40724: f64, t2482: f64, t2668: f64, t823: f64, t159: f64, t33127: f64, t64: f64, t222: f64, t124: f64, t138: f64, t40649: f64, t9645: f64, t810: f64, t240: f64, t9731: f64, t10293: f64, t212: f64, t800: f64, t820: f64, t849: f64, t9948: f64, t2699: f64, t2729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40725, t40731, t40735, t40737, t40757) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1345(t220, t40724, t2482, t2668, t823, t159, t33127, t64, t222, t124, t138, t40649, t9645);
        let (t40759, t40763, t40769, t40771, t40781, t40791) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1346(t40757, t810, t240, t9731, t10293, t124, t212, t800, t820, t849, t9948, t2699, t2729);
    (t40725, t40731, t40735, t40737, t40757, t40759, t40763, t40769, t40771, t40781, t40791)
}
