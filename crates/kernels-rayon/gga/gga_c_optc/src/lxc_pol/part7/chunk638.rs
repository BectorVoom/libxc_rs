//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 638/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk638(t1: f64, t3105: f64, t438: f64, t450: f64, t3138: f64, t466: f64, t429: f64, t530: f64, t321: f64, t457: f64, t3126: f64, t449: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3194 = t3105 * t1 * t438;
    let t3195 = t450 * t3194;
    let t3199 = 0.16793568152788065763e-2_f64 * t466 * t3138;
    let t3200 = t530 * t429;
    let t3201 = t321 * t3200;
    let t3203 = 0.19318136643975017455e-1_f64 * t457 * t3201;
    let t3204 = t449 * t3126;
    (t3194, t3195, t3199, t3200, t3203, t3204)
}
