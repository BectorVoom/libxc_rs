//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 536/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk536(t706: f64, t717: f64, t607: f64, t751: f64, t707: f64, t195: f64, t197: f64, t676: f64, t724: f64, t164: f64, t723: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2427 = t706 * t717;
    let t2430 = t751 * t607;
    let t2431 = t707 * t2430;
    let t2433 = 1.0_f64 / t195;
    let t2440 = 1.0_f64 / t197;
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    let t2459 = 1.0_f64 / t2458;
    let t2460 = t159 * t2459;
    (t2427, t2430, t2431, t2433, t2440, t2454, t2459, t2460)
}
