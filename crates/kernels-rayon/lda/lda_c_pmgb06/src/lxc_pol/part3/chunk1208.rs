//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1208/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1208(t13034: f64, t13038: f64, t13041: f64, t13046: f64, t13049: f64, t13056: f64, t13060: f64, t13063: f64, t13067: f64, t13071: f64, t13074: f64, t10711: f64, t13075: f64, t13076: f64, t13080: f64, t13081: f64, t13082: f64, t13084: f64, t13085: f64, t13088: f64, t13091: f64, t13093: f64, t13095: f64) -> (f64, f64) {
    let t14405 = -t13034 - t13038 - t13041 + t13046 - t13049 - t13056 - t13060 - t13063 + t13067 + t13071 + t13074;
    let t14406 = -t13075 - t13076 - t13080 - t13081 + t13082 + t13084 - t13085 + t13088 - t13091 - t13093 - t13095 + t10711;
    (t14405, t14406)
}
