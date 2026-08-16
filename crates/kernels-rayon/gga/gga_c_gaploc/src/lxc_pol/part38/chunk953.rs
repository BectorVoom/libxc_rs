//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 953/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk953(t46115: f64, t6717: f64, t6914: f64, t11426: f64, t20967: f64, t1: f64, t37975: f64, t1415: f64, t1457: f64, t2398: f64, t10463: f64, t10557: f64) -> (f64, f64, f64, f64, f64) {
    let t46118 = 0.12423108009070322895e3_f64 * t6914 * t6717 * t46115;
    let t46119 = t11426 * t20967;
    let t46121 = t37975 * t1;
    let t46125 = 0.42900587942220512003e1_f64 * t1415 * t46121 * t1457 * t2398;
    let t46127 = 0.85801175884441024006e1_f64 * t10557 * t10463;
    (t46118, t46119, t46121, t46125, t46127)
}
