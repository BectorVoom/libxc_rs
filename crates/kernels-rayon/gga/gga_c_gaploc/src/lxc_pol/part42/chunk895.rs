//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 895/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk895(t46044: f64, t2478: f64, t3536: f64, t6576: f64, t37977: f64, t44255: f64, t549: f64, t20367: f64, t44387: f64, t4820: f64, t2375: f64, t37575: f64) -> (f64, f64, f64, f64, f64) {
    let t46045 = 0.9585731488480187419e0_f64 * t46044;
    let t46047 = t6576 * t3536 * t2478;
    let t46052 = 0.47667319935800568892e0_f64 * t37977 * t549 * t44255;
    let t46055 = 0.23833659967900284446e0_f64 * t20367 * t4820 * t44387;
    let t46057 = 0.11916829983950142223e0_f64 * t37575 * t2375;
    (t46045, t46047, t46052, t46055, t46057)
}
