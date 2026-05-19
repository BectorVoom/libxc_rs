//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 130/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk130<F: Float>(t107: F, t110: F, t117: F, t118: F, t122: F, t123: F, t125: F, t191: F, t199: F, t202: F, t227: F, t24: F, t290: F, t295: F, t297: F, t302: F, t305: F, t312: F, t315: F, t317: F, t61: F, t77: F, t81: F, t84: F) -> F {
    let t321 = t24 * t77 + (-F::cast_from(0.031505407223141116_f64) * t84 * t118 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * t191 * t117) * t61 + (-F::cast_from(0.0837628205355044_f64) * t84 * t199 - F::cast_from(0.011938374665504766_f64) * t122 * t202 * t227 + F::cast_from(0.42708890021612717_f64) * t107 * t110 * t290) * t295 - F::cast_from(0.01197423401025461_f64) * t297 * t302 + (-F::cast_from(0.031835665774679375_f64) * t123 * t305 * t199 + F::cast_from(0.05332506774217938_f64) * t81 * t290) * t312 + F::cast_from(0.020267214298646783_f64) * t123 * t315 * t290 * t317;
    t321
}
