//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 130/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk130(t107: f64, t110: f64, t117: f64, t118: f64, t122: f64, t123: f64, t125: f64, t191: f64, t199: f64, t202: f64, t227: f64, t24: f64, t290: f64, t295: f64, t297: f64, t302: f64, t305: f64, t312: f64, t315: f64, t317: f64, t61: f64, t77: f64, t81: f64, t84: f64) -> f64 {
    let t321 = t24 * t77 + (-0.031505407223141116_f64 * t84 * t118 - 0.005388405304614574_f64 * t123 * t125 * t191 * t117) * t61 + (-0.0837628205355044_f64 * t84 * t199 - 0.011938374665504766_f64 * t122 * t202 * t227 + 0.42708890021612717_f64 * t107 * t110 * t290) * t295 - 0.01197423401025461_f64 * t297 * t302 + (-0.031835665774679375_f64 * t123 * t305 * t199 + 0.05332506774217938_f64 * t81 * t290) * t312 + 0.020267214298646783_f64 * t123 * t315 * t290 * t317;
    t321
}
