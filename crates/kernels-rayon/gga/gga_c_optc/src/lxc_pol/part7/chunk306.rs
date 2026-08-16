//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 306/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk306(t339: f64, t765: f64, t792: f64, t772: f64, t796: f64, t349: f64, t346: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t962 = 1.0_f64 / t339;
    let t966 = 0.19388333333333333333e1_f64 * t765;
    let t968 = 0.12315e-2_f64 * t792;
    let t970 = -t966 - 0.19388333333333333333e1_f64 * t772 - t968 - 0.12315e-2_f64 * t796;
    let t972 = t349 * t349;
    let t973 = 1.0_f64 / t972;
    let t974 = t346 * t973;
    let t975 = 0.72691666666666666667e3_f64 * t765;
    let t977 = 0.78666666666666666667e2_f64 * t792;
    let t979 = -t975 - 0.72691666666666666667e3_f64 * t772 - t977 - 0.78666666666666666667e2_f64 * t796;
    (t962, t970, t972, t973, t974, t979)
}
