//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 507/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk507(t1017: f64, t1021: f64, t1028: f64, t1038: f64, t1046: f64, t1093: f64, t1095: f64, t1107: f64, t1114: f64, t1115: f64, t1124: f64, t2142: f64, t283: f64, t975: f64) -> f64 {
    let t2147 = -0.00018311447306006544_f64 * t975 - t1021 + t1114 - t1028 + t1038 + t1046 + t1124 + 0.0197516734986138_f64 * t2142 * t283 - t1017 - t1107 - 4.0_f64 * t1115 + t1093 - 4.0_f64 * t1095;
    t2147
}
