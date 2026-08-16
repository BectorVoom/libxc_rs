//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 446/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk446(t2314: f64, t80: f64, t1094: f64, t1014: f64, t1015: f64, t1019: f64, t1020: f64, t2159: f64, t2163: f64, t2167: f64, t2171: f64, t2175: f64, t2179: f64) -> (f64, f64, f64) {
    let t2362 = t2314 * t80;
    let t2363 = t2362 * t1094;
    let t2378 = t1014 + t1015 + 4.431130547644593_f64 * t2159 + 4.431130547644593_f64 * t2163 - 4.431130547644593_f64 * t2167 + t1019 + t1020 + 0.2946275542389858_f64 * t2171 + 0.2946275542389858_f64 * t2175 - 0.2946275542389858_f64 * t2179;
    (t2362, t2363, t2378)
}
