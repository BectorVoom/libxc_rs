//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 54/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk54(t103: f64, t107: f64, t110: f64, t3: f64, t34: f64, t55: f64, t93: f64) -> f64 {
    let t113 = 1.0_f64 - t93 * t55 * t34 / 4.0_f64 + 0.0204825_f64 * t103 - 0.0030486129349252553_f64 * t3 + 0.0003485625_f64 * t107 * t110;
    t113
}
