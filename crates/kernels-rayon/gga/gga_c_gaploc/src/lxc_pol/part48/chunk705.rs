//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 705/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk705(t13382: f64, t13418: f64, t13456: f64, t13481: f64, t502: f64, t11595: f64, t948: f64, t2508: f64, t3650: f64, t7301: f64, t943: f64, t11613: f64, t2624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13483 = t13382 + t13418 + t13456 + t13481;
    let t13484 = t502 * t13483;
    let t13486 = t11595 * t948;
    let t13488 = 0.23071578690426672851e-1_f64 * t2508 * t13486;
    let t13489 = t3650 * t7301;
    let t13490 = t943 * t13489;
    let t13492 = t11613 * t2624;
    (t13483, t13484, t13486, t13488, t13489, t13490, t13492)
}
