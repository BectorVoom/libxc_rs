//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta943 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3096;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta943(t1732: f64, t3433: f64, t69591: f64, t20644: f64, t5104: f64, t5068: f64, t68792: f64, t5109: f64, t68952: f64, t17092: f64, t20641: f64, t16840: f64, t20645: f64, t20580: f64, t58342: f64, t20648: f64, t20652: f64, t58473: f64, t1149: f64, t12227: f64, t24262: f64, t12248: f64, t6474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81618, t81621, t81623, t81625, t81627, t81629) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3096(t1732, t3433, t69591, t20644, t5104, t5068, t68792, t5109, t68952, t17092, t20641, t16840, t20645);
        let (t81631, t81633, t81635, t81638, t81641) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3097(t20580, t58342, t16840, t20648, t20652, t58473, t1149, t12227, t24262, t12248, t5104, t6474);
    (t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633, t81635, t81638, t81641)
}
