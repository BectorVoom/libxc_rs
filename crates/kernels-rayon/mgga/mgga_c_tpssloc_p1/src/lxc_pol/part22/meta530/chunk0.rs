//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2001/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2001(t85: f64, t24: f64, t10276: f64, t73: f64, t11152: f64, t76: f64, t41: f64, t42: f64, t53: f64, t54: f64, t9576: f64, t2405: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39096 = 1.0_f64 / t73 / t10276;
    let t39114 = 1.0_f64 / t76 / t11152;
    let t39157 = t41 * t41;
    let t39159 = 1.0_f64 / t42 / t39157;
    let t39166 = t53 * t53;
    let t39168 = 1.0_f64 / t54 / t39166;
    let t39210 = 20944.0_f64 / 81.0_f64 * t9576;
    let t39246 = t2405 * t2405;
    (t39063, t39096, t39114, t39159, t39168, t39210, t39246)
}
