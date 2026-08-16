//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 563/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk563(t1366: f64, t1372: f64, t186: f64, t315: f64, t934: f64, t1375: f64, t526: f64, t955: f64, t163: f64, t497: f64, t147: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3331 = 0.21642082724729686_f64 * t1372 * t1366;
    let t3333 = t934 * t315 * t186;
    let t3335 = 0.011181742741110338_f64 * t1375 * t3333;
    let t3350 = t955 * t526;
    let t3357 = 1.0_f64 / t163 / t497;
    let t3358 = t147 * t3357;
    let t3365 = t740 * t147;
    (t3331, t3333, t3335, t3350, t3357, t3358, t3365)
}
