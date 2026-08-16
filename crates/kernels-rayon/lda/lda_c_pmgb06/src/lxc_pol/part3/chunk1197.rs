//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1197/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1197(t12184: f64, t12186: f64, t12189: f64, t12192: f64, t12197: f64, t12199: f64, t12201: f64, t12203: f64, t12208: f64, t12210: f64, t12219: f64, t1377: f64, t2342: f64, t97: f64) -> (f64, f64) {
    let t14345 = -t12184 - t12186 - t12189 + t12192 - t12197 - t12199 - t12201 - t12203 + t12208 + t12210 - t12219;
    let t14347 = t2342 * t97 * t1377;
    (t14345, t14347)
}
