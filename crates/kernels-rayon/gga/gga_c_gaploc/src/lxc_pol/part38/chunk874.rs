//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 874/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk874(t13542: f64, t731: f64, t11613: f64, t2508: f64, t7659: f64, t37032: f64, t7663: f64, t13489: f64, t13495: f64, t7137: f64, t13486: f64, t7129: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44920 = t731 * t13542;
    let t44921 = 0.42725145723012357132e-3_f64 * t44920;
    let t44924 = 0.38452631150711121418e0_f64 * t2508 * t11613 * t7659;
    let t44927 = 0.46143157380853345701e0_f64 * t2508 * t37032 * t7663;
    let t44928 = t731 * t13489;
    let t44931 = 0.10254034973522965712e-1_f64 * t7137 * t13495;
    let t44933 = 0.23071578690426672851e-1_f64 * t7129 * t13486;
    (t44921, t44924, t44927, t44928, t44931, t44933)
}
