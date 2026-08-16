//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1153/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1153(t23: f64, t2548: f64, t191: f64, t8386: f64, t2433: f64, t176: f64, t8378: f64, t998: f64, t2364: f64, t7278: f64, t2562: f64, t7274: f64, t999: f64) -> (f64, f64, f64, f64) {
    let t23982 = t23 * t2548;
    let t23983 = t23982 * t191;
    let t23984 = t23983 * t8386;
    let t23985 = t2433 * t23984;
    let t23990 = t176 * t8378 * t998;
    let t23993 = t2364 * t7278;
    let t23996 = t999 * t7274 * t2562;
    (t23985, t23990, t23993, t23996)
}
