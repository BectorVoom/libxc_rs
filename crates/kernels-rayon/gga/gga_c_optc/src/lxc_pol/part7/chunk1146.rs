//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1146/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1146(t23859: f64, t23872: f64, t787: f64, t23543: f64, t23545: f64, t23551: f64, t23553: f64, t23555: f64, t23557: f64, t23561: f64, t23565: f64, t23567: f64, t23569: f64, t23840: f64, t23842: f64, t23846: f64) -> (f64, f64, f64) {
    let t23873 = t23859 + t23872;
    let t23874 = t787 * t23873;
    let t23882 = -0.18396666666666666667e0_f64 * t23543 - 0.44152e0_f64 * t23545 + 0.44152e0_f64 * t23551 + 0.98115555555555555556e0_f64 * t23553 + 0.247573125e0_f64 * t23840 - 0.3883875e1_f64 * t23842 + 0.6189328125e-1_f64 * t23846 + 0.16504875e0_f64 * t23874 + 0.11038e1_f64 * t23555 + 0.132456e1_f64 * t23557 - 0.99342e0_f64 * t23561 - 0.82785e-1_f64 * t23565 + 0.22076e0_f64 * t23567 + 0.98115555555555555555e-1_f64 * t23569;
    (t23873, t23874, t23882)
}
