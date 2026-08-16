//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 370/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk370(t2283: f64, t33: f64, t40: f64, t632: f64, t73: f64, t52: f64, t636: f64, t76: f64, t2244: f64, t2250: f64, t634: f64, t638: f64) -> (f64, f64, f64, f64) {
    let t2284 = t33 * t2283;
    let t2289 = t632 * t40;
    let t2291 = 1.0_f64 / t73 / t2289;
    let t2296 = t636 * t52;
    let t2298 = 1.0_f64 / t76 / t2296;
    let t2303 = 28.0_f64 / 9.0_f64 * t2291 * t2244 - 4.0_f64 / 3.0_f64 * t634 * t2250 + 28.0_f64 / 9.0_f64 * t2298 * t2244 + 4.0_f64 / 3.0_f64 * t638 * t2250;
    (t2284, t2289, t2296, t2303)
}
