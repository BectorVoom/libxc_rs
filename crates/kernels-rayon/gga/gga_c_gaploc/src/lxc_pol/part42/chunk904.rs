//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 904/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk904(t11408: f64, t1445: f64, t1562: f64, t2293: f64, t10252: f64, t10615: f64, t900: f64, t20884: f64, t37667: f64, t13397: f64, t587: f64, t589: f64) -> (f64, f64, f64, f64) {
    let t46233 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t11408 * t2293;
    let t46235 = t10615 * t900 * t10252;
    let t46237 = t37667 * t20884;
    let t46240 = t587 * t589 * t13397;
    (t46233, t46235, t46237, t46240)
}
