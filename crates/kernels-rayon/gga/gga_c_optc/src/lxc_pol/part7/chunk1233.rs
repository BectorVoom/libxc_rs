//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1233/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1233(t2606: f64, t864: f64, t14330: f64, t7178: f64, t25423: f64, t3906: f64, t19: f64, t25425: f64, t2662: f64, t2264: f64, t7982: f64, t2670: f64, t8384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25440 = t864 * t2606;
    let t25445 = t14330 * t7178;
    let t25453 = t3906 * t25423;
    let t25454 = t25425 * t19;
    let t25458 = t2662 * t25423;
    let t25468 = t7982 * t2264;
    let t25472 = t8384 * t2670;
    (t25440, t25445, t25453, t25454, t25458, t25468, t25472)
}
