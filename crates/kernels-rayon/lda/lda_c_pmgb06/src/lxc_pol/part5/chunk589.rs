//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 589/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk589(t1122: f64, t3969: f64, t1139: f64, t301: f64, t413: f64, t1183: f64, t718: f64, t247: f64, t398: f64, t113: f64, t1135: f64, t100: f64, t641: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3970 = t3969 * t1122;
    let t3987 = t1139 * t413 * t301;
    let t3991 = 0.0008717022455366076_f64 * t718 * t1183 * t301;
    let t3993 = t247 * t398;
    let t3995 = t3993 * t113 * t301;
    let t3999 = 0.004067943812504169_f64 * t1135 * t413 * t301;
    let t4001 = 1.0_f64 / t100 / t641;
    (t3970, t3987, t3991, t3993, t3995, t3999, t4001)
}
