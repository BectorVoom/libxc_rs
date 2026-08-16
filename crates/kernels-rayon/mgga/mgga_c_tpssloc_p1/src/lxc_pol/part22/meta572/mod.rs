//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2080;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta572(t43603: f64, t68: f64, t3215: f64, t3399: f64, t3402: f64, t3639: f64, t11545: f64, t241: f64, t3241: f64, t242: f64, t281: f64, t415: f64, t2296: f64, t11778: f64, t154: f64, t1091: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43604, t43637, t43689, t43692, t43706, t43761, t43763, t43776) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2080(t43603, t68, t3215, t3399, t3402, t3639, t11545, t241, t3241, t242, t281, t415);
        let (t43777, t43791, t43809, t43816) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2081(t43776, t2296, t3241, t11778, t154, t1091, t9698);
    (t43604, t43637, t43689, t43692, t43706, t43761, t43763, t43776, t43777, t43791, t43809, t43816)
}
