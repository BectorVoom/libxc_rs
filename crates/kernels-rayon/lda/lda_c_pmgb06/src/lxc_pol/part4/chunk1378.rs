//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1378/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1378(t107: f64, t410: f64, t6104: f64, t122: f64, t1669: f64, t2659: f64, t11744: f64, t11747: f64, t1200: f64, t14231: f64, t14233: f64, t14235: f64, t14237: f64, t14240: f64, t1799: f64, t18066: f64, t1808: f64, t199: f64, t2454: f64, t5543: f64, t566: f64, t6928: f64, t868: f64) -> f64 {
    let t18141 = t107 * t410 * t6104;
    let t18144 = t122 * t1669 * t2659;
    let t18151 = -0.3350512821420176_f64 * t1799 * t1808 + 5.314884091578472_f64 * t11744 - 8.858140152630787_f64 * t11747 - 0.0837628205355044_f64 * t18066 * t199 - 0.1675256410710088_f64 * t6928 * t566 - 0.0837628205355044_f64 * t2454 * t1200 - 0.1675256410710088_f64 * t5543 * t868 - 1.1389037339096726_f64 * t18141 - 0.053059442957798957_f64 * t18144 - 0.6701025642840353_f64 * t14231 - 0.6701025642840353_f64 * t14233 - 0.6701025642840353_f64 * t14235 - 0.6701025642840353_f64 * t14237 + 0.1675256410710088_f64 * t14240;
    t18151
}
