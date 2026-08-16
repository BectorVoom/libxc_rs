//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2015/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2015(t604: f64, t9226: f64, t2233: f64, t2239: f64, t601: f64, t9238: f64, t85: f64, t24: f64, t10276: f64, t73: f64, t11152: f64, t76: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39046 = t9226 * t604;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39096 = 1.0_f64 / t73 / t10276;
    let t39114 = 1.0_f64 / t76 / t11152;
    (t39046, t39049, t39054, t39063, t39096, t39114)
}
