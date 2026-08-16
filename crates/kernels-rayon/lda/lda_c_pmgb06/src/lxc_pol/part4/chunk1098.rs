//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1098/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1098(t161: f64, t489: f64, t5416: f64, t1499: f64, t1933: f64, t4790: f64, t486: f64, t1447: f64, t5359: f64, t1902: f64, t3213: f64, t5494: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13235 = t161 * t489 * t5416;
    let t13237 = t1499 * t1933;
    let t13239 = t486 * t4790;
    let t13241 = t1447 * t5359;
    let t13243 = t3213 * t1902;
    let t13245 = t1447 * t5494;
    (t13235, t13237, t13239, t13241, t13243, t13245)
}
