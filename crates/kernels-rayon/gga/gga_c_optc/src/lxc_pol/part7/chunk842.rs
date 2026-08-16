//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 842/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk842(t7274: f64, t916: f64, t913: f64, t2573: f64, t909: f64, t911: f64, t2367: f64, t2602: f64, t930: f64, t7398: f64, t914: f64, t7882: f64, t953: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8068 = t7274 * t916;
    let t8069 = t913 * t8068;
    let t8072 = t909 * t2573 * t911;
    let t8075 = t2367 * t2602;
    let t8076 = t930 * t8075;
    let t8078 = t914 * t7398;
    let t8083 = t953 * t7882;
    (t8068, t8069, t8072, t8075, t8076, t8078, t8083)
}
