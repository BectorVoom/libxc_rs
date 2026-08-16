//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 208/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk208(t107: f64, t290: f64, t410: f64, t110: f64, t242: f64, t30: f64, t238: f64, t232: f64, t27: f64, t347: f64, t402: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t616 = 0.5694518669548363_f64 * t107 * t410 * t290;
    let t619 = 0.0011073470983333333_f64 * t30 * t110 * t242;
    let t620 = t238 * t238;
    let t621 = 1.0_f64 / t620;
    let t622 = t232 * t621;
    let t623 = t347 * t27;
    let t624 = t623 * t402;
    let t627 = f64::sqrt(t36);
    (t616, t619, t620, t621, t622, t623, t624, t627)
}
