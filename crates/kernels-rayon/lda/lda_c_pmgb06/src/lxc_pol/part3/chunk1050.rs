//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1050/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1050(t12485: f64, t5138: f64, t5139: f64, t11904: f64, t5072: f64, t11903: f64, t5137: f64, t5140: f64, t1414: f64, t1639: f64, t5068: f64, t5071: f64) -> (f64, f64, f64, f64) {
    let t12491 = t5138 * t5139 * t12485 / 9.0_f64;
    let t12493 = 4.0_f64 / 15.0_f64 * t11904 * t5072;
    let t12494 = t11903 * t5137;
    let t12496 = 2.0_f64 / 9.0_f64 * t12494 * t5140;
    let t12497 = t1639 * t1414;
    let t12500 = 4.0_f64 / 15.0_f64 * t5068 * t12497 * t5071;
    (t12491, t12493, t12496, t12500)
}
