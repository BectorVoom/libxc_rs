//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 242/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk242(t191: f64, t933: f64, t332: f64, t786: f64, t330: f64, t197: f64, t325: f64, t641: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t934 = t933 * t191;
    let t935 = t332 * t786;
    let t936 = t330 * t935;
    let t937 = t197 * t936;
    let t940 = t325 * t641;
    let t941 = t332 * t6;
    (t934, t935, t936, t937, t940, t941)
}
