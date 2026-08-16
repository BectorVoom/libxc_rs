//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 813/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk813(t1406: f64, t6715: f64, t1564: f64, t588: f64, t16879: f64, t486: f64, t165: f64, t2089: f64, t16534: f64, t169: f64, t10913: f64, t2021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21370 = t1406 * t6715;
    let t21373 = t588 * t1564;
    let t21501 = t16879 * t486;
    let t21502 = t165 * t2089;
    let t22090 = t16534 * t169;
    let t22242 = t2021 * t10913;
    (t21370, t21373, t21501, t21502, t22090, t22242)
}
