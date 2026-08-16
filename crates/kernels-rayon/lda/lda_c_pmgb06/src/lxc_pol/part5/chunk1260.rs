//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1260/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1260(t1799: f64, t1808: f64, t18418: f64, t18420: f64, t18422: f64, t18424: f64, t18426: f64, t18428: f64, t18430: f64, t18432: f64, t18434: f64, t2422: f64, t2454: f64, t6939: f64, t795: f64) -> f64 {
    let t22113 = -0.2512884616065132_f64 * t2454 * t1808 - 0.2512884616065132_f64 * t1799 * t2422 - 0.2512884616065132_f64 * t795 * t6939 - 1.0051538464260528_f64 * t18418 - 0.5025769232130264_f64 * t18420 - 0.5025769232130264_f64 * t18422 + 0.5025769232130264_f64 * t18424 + 0.2512884616065132_f64 * t18426 + 0.2512884616065132_f64 * t18428 + 0.2512884616065132_f64 * t18430 + 0.2512884616065132_f64 * t18432 + 0.5025769232130264_f64 * t18434;
    t22113
}
