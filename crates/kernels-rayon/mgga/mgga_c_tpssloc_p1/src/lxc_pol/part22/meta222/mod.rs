//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1271;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta222(t9216: f64, t14: f64, t598: f64, t2230: f64, t594: f64, t2229: f64, t3: f64, t19: f64, t2239: f64, t601: f64, t83: f64, t84: f64, t85: f64, t24: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9217, t9218) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1271(t9216, t14, t598);
        let (t9219, t9220, t9221, t9223, t9225, t9231, t9238, t9239) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1272(t9218, t2230, t594, t2229, t3, t19, t2239, t601, t83, t84, t85, t24);
    (t9217, t9218, t9219, t9220, t9221, t9223, t9225, t9231, t9238, t9239)
}
