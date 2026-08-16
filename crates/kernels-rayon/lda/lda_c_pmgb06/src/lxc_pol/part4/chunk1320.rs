//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1320/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1320(t13399: f64, t13407: f64, t17164: f64, t17166: f64, t17169: f64, t17172: f64, t17175: f64, t17177: f64, t17190: f64, t17193: f64, t9530: f64, t9532: f64, t9552: f64) -> f64 {
    let t17361 = -0.0013993827160493828_f64 * t17164 + 0.01847185185185185_f64 * t17166 + 0.002099074074074074_f64 * t17169 + 0.005597530864197531_f64 * t17172 - 0.007556666666666666_f64 * t17175 - 0.007556666666666666_f64 * t17177 - 0.007556666666666666_f64 * t17190 + 0.011335_f64 * t17193 - 0.059613703703703703_f64 * t13399 - 0.003918271604938271_f64 * t13407 + 0.0008396296296296296_f64 * t9530 + 0.000559753086419753_f64 * t9532 - 0.003918271604938271_f64 * t9552;
    t17361
}
