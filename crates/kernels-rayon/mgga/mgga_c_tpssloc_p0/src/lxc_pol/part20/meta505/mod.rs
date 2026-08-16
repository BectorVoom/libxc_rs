//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2015;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta505(t604: f64, t9226: f64, t2233: f64, t2239: f64, t601: f64, t9238: f64, t85: f64, t24: f64, t10276: f64, t73: f64, t11152: f64, t76: f64, t41: f64, t42: f64, t53: f64, t54: f64, t9576: f64, t111: f64, t9346: f64, t2405: f64, t2420: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39046, t39049, t39054, t39063, t39096, t39114) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2015(t604, t9226, t2233, t2239, t601, t9238, t85, t24, t10276, t73, t11152, t76);
        let (t39159, t39168, t39210, t39235, t39246, t39249) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2016(t41, t42, t53, t54, t9576, t111, t9346, t2405, t2420, t702);
    (t39046, t39049, t39054, t39063, t39096, t39114, t39159, t39168, t39210, t39235, t39246, t39249)
}
