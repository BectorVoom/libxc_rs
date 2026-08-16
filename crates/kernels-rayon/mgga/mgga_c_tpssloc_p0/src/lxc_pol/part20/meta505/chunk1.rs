//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2016/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2016(t41: f64, t42: f64, t53: f64, t54: f64, t9576: f64, t111: f64, t9346: f64, t2405: f64, t2420: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39157 = t41 * t41;
    let t39159 = 1.0_f64 / t42 / t39157;
    let t39166 = t53 * t53;
    let t39168 = 1.0_f64 / t54 / t39166;
    let t39210 = 20944.0_f64 / 81.0_f64 * t9576;
    let t39235 = t9346 * t111;
    let t39246 = t2405 * t2405;
    let t39249 = 6.0_f64 * t2420 * t39246 * t702;
    (t39159, t39168, t39210, t39235, t39246, t39249)
}
