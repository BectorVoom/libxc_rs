//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1813/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1813(t1198: f64, t12571: f64, t3531: f64, t3539: f64, t3543: f64, t3535: f64, t12485: f64, t12487: f64, t3523: f64, t1196: f64, t1298: f64, t3798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12573 = 0.17544670867903938621e1_f64 * t12571 * t1198;
    let t12575 = 0.17544670867903938621e1_f64 * t3531 * t3539;
    let t12577 = 0.51947577317044391276e2_f64 * t3531 * t3543;
    let t12579 = 0.35089341735807877242e1_f64 * t3531 * t3535;
    let t12581 = t12485 * t12487 * t3523;
    let t12583 = 0.10389515463408878255e3_f64 * t1196 * t12581;
    let t12584 = t3798 * t1298;
    (t12573, t12575, t12577, t12579, t12581, t12583, t12584)
}
