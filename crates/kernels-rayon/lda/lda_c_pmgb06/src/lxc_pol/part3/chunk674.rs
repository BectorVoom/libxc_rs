//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 674/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk674(t4065: f64, t4071: f64, t4122: f64, t4152: f64, t4155: f64, t4156: f64, t4158: f64, t4166: f64, t122: f64, t1669: f64, t610: f64, t1735: f64, t569: f64) -> (f64, f64, f64) {
    let t4169 = t4065 + t4071 + t4122 + t4152 + t4155 + t4156 + t4158 + t4166;
    let t4174 = t122 * t1669 * t610;
    let t4177 = t122 * t569 * t1735;
    (t4169, t4174, t4177)
}
