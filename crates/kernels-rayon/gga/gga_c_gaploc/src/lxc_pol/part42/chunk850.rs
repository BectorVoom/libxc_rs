//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 850/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk850(t1960: f64, t2728: f64, t3684: f64, t11711: f64, t23555: f64, t10298: f64, t8045: f64, t2902: f64, t3366: f64, t4349: f64, t11701: f64, t11556: f64, t2355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45144 = 2.0_f64 * t1960 * t3684 * t2728;
    let t45146 = 6.0_f64 * t23555 * t11711;
    let t45148 = 4.0_f64 * t8045 * t10298;
    let t45151 = 12.0_f64 * t4349 * t3366 * t2902;
    let t45163 = t11701 * t2728;
    let t45164 = t2355 * t11556;
    (t45144, t45146, t45148, t45151, t45163, t45164)
}
