//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1081/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1081(t6563: f64, t740: f64, t6602: f64, t6607: f64, t6763: f64, t6766: f64, t172: f64, t1879: f64, t22052: f64, t22610: f64, t22721: f64, t22724: f64, t22726: f64, t22728: f64, t3539: f64, t606: f64, t616: f64, t6560: f64, t95: f64) -> f64 {
    let t23435 = t6563 * t740;
    let t23438 = 14.0_f64 / 3.0_f64 * t6602 * t740;
    let t23439 = t6607 * t740;
    let t23441 = t6763 * t6766;
    let t23452 = -14.0_f64 / 3.0_f64 * t23435 - t23438 - 14.0_f64 * t23439 - t22721 + t22724 + 0.62027715443768233192e-1_f64 * t1879 * t23441 * t616 + 0.62027715443768233192e-1_f64 * t3539 * t172 * t6560 * t616 + t22726 + t22610 - t22728 + 0.77534644304710291488e-2_f64 * t95 * t606 * t22052;
    t23452
}
