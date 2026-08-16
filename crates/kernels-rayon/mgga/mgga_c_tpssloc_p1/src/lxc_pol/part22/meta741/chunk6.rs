//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2449/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2449(t43002: f64, t48156: f64, t48158: f64, t60163: f64, t60168: f64, t60173: f64, t60192: f64, t60194: f64, t60202: f64, t60204: f64, t60274: f64, t60308: f64, t60310: f64, t60312: f64, t68545: f64, t68549: f64, t68552: f64, t68556: f64, t68563: f64, t68649: f64) -> f64 {
    let t69615 = -4.0_f64 * t68545 + 3.0_f64 * t68549 + 2.0_f64 * t68552 - t68556 - t60163 / 3.0_f64 - 10.0_f64 / 9.0_f64 * t60168 + 5.0_f64 / 9.0_f64 * t60173 + 2.0_f64 / 9.0_f64 * t68563 - t48156 + t48158 - 2.0_f64 * t60192 + 4.0_f64 / 3.0_f64 * t60194 + 2.0_f64 / 3.0_f64 * t60202 + 5.0_f64 / 27.0_f64 * t60204 - t43002 - t60274 / 9.0_f64 + t68649 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t60308 - 2.0_f64 / 9.0_f64 * t60310 - 4.0_f64 / 27.0_f64 * t60312;
    t69615
}
