//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 881/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk881(t208: f64, t213: f64, t4641: f64, t4913: f64, t83: f64, t4076: f64, t588: f64, t97: f64, t4093: f64, t138: f64, t163: f64, t9175: f64) -> (f64, f64, f64, f64) {
    let t9478 = t83 * (-0.33530864197530863_f64 * t4641 + 1.8360493827160493_f64 * t4913) * t208 * t213 / 3.0_f64;
    let t9481 = 0.2431111111111111_f64 * t4076 * t97 * t588;
    let t9483 = t4093 * t97 * t588;
    let t9501 = t138 * t9175 * t163;
    (t9478, t9481, t9483, t9501)
}
