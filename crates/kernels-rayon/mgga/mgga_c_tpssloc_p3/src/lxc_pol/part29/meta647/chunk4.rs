//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2146/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2146(t12971: f64, t6552: f64, t6637: f64, t6638: f64, t22893: f64, t23164: f64, t25312: f64, t82011: f64, t1888: f64, t232: f64, t47425: f64, t6646: f64) -> (f64, f64, f64, f64) {
    let t87676 = t6552 * t6637 * t6638 * t12971;
    let t87679 = t23164 * t22893 * t25312;
    let t87680 = 0.16449340668482264365e-1_f64 * t87679;
    let t87687 = 0.12793931631041761173e0_f64 * t82011;
    let t87692 = t1888 * t6646 * t47425 * t232;
    (t87676, t87680, t87687, t87692)
}
