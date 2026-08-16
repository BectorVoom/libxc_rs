//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 522/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk522(t377: f64, t783: f64, t384: f64, t787: f64, t2214: f64, t69: f64, t55: f64, t68: f64) -> (f64, f64, f64, f64) {
    let t2238 = t783 * t377;
    let t2241 = t787 * t384;
    let t2245 = t69 * t2214;
    let t2247 = t68 * t55;
    (t2238, t2241, t2245, t2247)
}
