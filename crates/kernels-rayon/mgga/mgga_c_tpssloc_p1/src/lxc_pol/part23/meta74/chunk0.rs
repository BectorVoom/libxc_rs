//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 439/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk439(t2281: f64, t40: f64, t632: f64, t73: f64, t52: f64, t636: f64, t76: f64, t107: f64, t106: f64, t655: f64, t94: f64, t102: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2282 = 88.0_f64 / 9.0_f64 * t2281;
    let t2289 = t632 * t40;
    let t2291 = 1.0_f64 / t73 / t2289;
    let t2296 = t636 * t52;
    let t2298 = 1.0_f64 / t76 / t2296;
    let t2327 = 11.0_f64 / 9.0_f64 * t2281 * t107;
    let t2331 = 1.0_f64 / t655 / t106;
    let t2341 = 1.0_f64 / t94;
    let t2349 = 1.0_f64 / t102;
    (t2282, t2289, t2291, t2296, t2298, t2327, t2331, t2341, t2349)
}
