//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1040/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1040(t2595: f64, t7256: f64, t23548: f64, t7253: f64, t2548: f64, t7298: f64, t864: f64, t9: f64, t334: f64, t7946: f64, t317: f64, t509: f64, t896: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25093 = t2595 * t7256;
    let t25121 = t7253 * t23548;
    let t25174 = t2548 * t7256;
    let t25183 = t864 * t7298;
    let t25217 = t9 * t2595;
    let t25277 = 1.0_f64 / t7946 / t334;
    let t25278 = t317 * t25277;
    let t25412 = t509 * t896;
    (t25093, t25121, t25174, t25183, t25217, t25278, t25412)
}
