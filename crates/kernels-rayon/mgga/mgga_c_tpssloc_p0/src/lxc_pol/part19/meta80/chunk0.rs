//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 468/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk468(t182: f64, t2448: f64, t676: f64, t724: f64, t164: f64, t723: f64, t159: f64, t730: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2450 = 0.19751673498613801407e-1_f64 * t2448 * t182;
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    let t2459 = 1.0_f64 / t2458;
    let t2460 = t159 * t2459;
    let t2461 = t730 * t730;
    (t2450, t2454, t2458, t2459, t2460, t2461)
}
