//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 790/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk790(t285: f64, t7402: f64, t248: f64, t3700: f64, t3707: f64, t3713: f64, t3719: f64, t3727: f64, t3731: f64, t3736: f64, t3744: f64, t3762: f64, t4532: f64, t4534: f64, t6079: f64) -> (f64, f64) {
    let t7414 = t7402 * t285;
    let t7416 = t3700 - 0.0005493434191801964_f64 * t6079 + 0.0007324578922402618_f64 * t4532 - 24.0_f64 * t4534 + t248 * t7414 - t3707 + t3713 + t3719 - t3727 + t3731 - t3736 - t3744 - t3762;
    (t7414, t7416)
}
