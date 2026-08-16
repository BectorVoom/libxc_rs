//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 626/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk626(t5166: f64, t281: f64, t226: f64, t1248: f64, t1249: f64, t1253: f64) -> (f64, f64, f64) {
    let t5167 = 8.0_f64 * t5166;
    let t5168 = t281 * t281;
    let t5169 = 1.0_f64 / t5168;
    let t5170 = t226 * t5169;
    let t5177 = t1248 * t1253 * t1249;
    (t5167, t5170, t5177)
}
