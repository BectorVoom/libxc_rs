//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 330/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk330(t1121: f64, t1122: f64, t1017: f64, t1021: f64, t1028: f64, t1038: f64, t1046: f64, t1089: f64, t1107: f64, t1110: f64, t1114: f64, t1115: f64, t1118: f64, t283: f64) -> (f64, f64) {
    let t1124 = 0.01084358130030174_f64 * t1121 * t1122;
    let t1125 = t1107 - t1110 - t1017 + t1114 - t1021 + 8.0_f64 * t1115 + t1118 - t1028 + 0.0197516734986138_f64 * t1089 * t283 + t1038 + t1046 + t1124;
    (t1124, t1125)
}
