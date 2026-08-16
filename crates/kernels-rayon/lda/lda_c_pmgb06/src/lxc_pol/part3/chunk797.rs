//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 797/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk797(t1795: f64, t566: f64, t3: f64, t4463: f64, t1329: f64, t868: f64, t1808: f64, t391: f64, t1200: f64, t1338: f64, t1799: f64, t199: f64, t399: f64, t4187: f64, t4212: f64, t4214: f64, t4216: f64, t4218: f64, t4220: f64, t4435: f64, t795: f64, t84: f64) -> (f64, f64) {
    let t5542 = 0.1675256410710088_f64 * t1795 * t566;
    let t5543 = t3 * t4463;
    let t5551 = 0.1675256410710088_f64 * t1329 * t868;
    let t5553 = 0.1675256410710088_f64 * t391 * t1808;
    let t5563 = -0.0837628205355044_f64 * t84 * t4435 + t5542 - 0.0837628205355044_f64 * t5543 * t199 - 0.1675256410710088_f64 * t1799 * t566 - 0.0837628205355044_f64 * t795 * t1200 + t5551 + t5553 - 0.0837628205355044_f64 * t1338 * t868 - 0.1675256410710088_f64 * t399 * t1808 + t4187 - 0.3350512821420176_f64 * t4212 - 0.3350512821420176_f64 * t4214 + 0.0837628205355044_f64 * t4216 + 0.1675256410710088_f64 * t4218 + 0.0837628205355044_f64 * t4220;
    (t5543, t5563)
}
