//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 984/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk984(t1329: f64, t1808: f64, t391: f64, t4435: f64, t1200: f64, t1795: f64, t11589: f64, t3: f64, t107: f64, t110: f64, t11188: f64, t11695: f64, t1338: f64, t1799: f64, t199: f64, t2804: f64, t399: f64, t5543: f64, t566: f64, t84: f64, t868: f64) -> (f64, f64) {
    let t11698 = t1329 * t1808;
    let t11700 = t391 * t4435;
    let t11708 = t1795 * t1200;
    let t11710 = t3 * t11589;
    let t11717 = 0.42708890021612717_f64 * t107 * t110 * t11188 - 0.0837628205355044_f64 * t84 * t11695 + 0.5025769232130264_f64 * t11698 + 0.2512884616065132_f64 * t11700 - 0.0837628205355044_f64 * t2804 * t868 - 0.2512884616065132_f64 * t1338 * t1808 - 0.2512884616065132_f64 * t399 * t4435 + 0.2512884616065132_f64 * t11708 - 0.0837628205355044_f64 * t11710 * t199 - 0.2512884616065132_f64 * t5543 * t566 - 0.2512884616065132_f64 * t1799 * t1200;
    (t11710, t11717)
}
