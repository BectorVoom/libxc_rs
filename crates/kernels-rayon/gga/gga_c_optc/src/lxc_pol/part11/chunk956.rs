//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 956/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk956(t11671: f64, t14885: f64, t14887: f64, t14889: f64, t17389: f64, t17392: f64, t17406: f64, t17409: f64, t17419: f64, t9268: f64, t9269: f64, t11677: f64, t14881: f64, t14883: f64, t14895: f64, t17338: f64, t17342: f64, t17346: f64, t17350: f64, t17354: f64, t17358: f64, t17412: f64) -> (f64, f64) {
    let t17561 = -0.2585111111111111111e1_f64 * t11671 - 0.12315e-2_f64 * t17419 - t9268 - t9269 - 0.38776666666666666665e1_f64 * t14887 + 0.19388333333333333333e1_f64 * t14889 + 0.12925555555555555555e1_f64 * t14885 + 0.2463e-2_f64 * t17406 - 0.12315e-2_f64 * t17389 - 0.7389e-2_f64 * t17409 + 0.7389e-2_f64 * t17392;
    let t17573 = -0.21542592592592592592e1_f64 * t17338 - 0.19388333333333333333e1_f64 * t17358 + 0.11633e2_f64 * t17354 + 0.77553333333333333331e1_f64 * t17342 - 0.38776666666666666665e1_f64 * t17346 - 0.11633e2_f64 * t17350 - 0.54733333333333333333e-3_f64 * t17412 - 0.4105e-2_f64 * t11677 + 0.821e-3_f64 * t14895 - 0.4926e-2_f64 * t14881 + 0.2463e-2_f64 * t14883;
    (t17561, t17573)
}
