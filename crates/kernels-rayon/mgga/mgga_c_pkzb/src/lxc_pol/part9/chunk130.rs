//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 130/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk130(t369: f64, t378: f64, t237: f64, t354: f64, t356: f64, t365: f64, t23: f64, t275: f64) -> (f64, f64, f64) {
    let t379 = t369 * t378;
    let t382 = t237 * (-0.310907e-1_f64 * t356 * t365 + t354 - 0.19751673498613801407e-1_f64 * t379);
    let t384 = 0.19751673498613801407e-1_f64 * t237 * t379;
    let t385 = t23 * t275;
    (t382, t384, t385)
}
