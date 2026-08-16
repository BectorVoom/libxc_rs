//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1445/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1445(t10494: f64, t1338: f64, t18418: f64, t18420: f64, t18422: f64, t18424: f64, t18426: f64, t18428: f64, t18430: f64, t18432: f64, t18434: f64, t18437: f64, t2422: f64, t399: f64, t4435: f64, t6939: f64, t795: f64, t84: f64) -> f64 {
    let t18440 = 1.0051538464260528_f64 * t10494 - 0.1675256410710088_f64 * t795 * t4435 - 0.0837628205355044_f64 * t1338 * t2422 - 0.1675256410710088_f64 * t399 * t6939 - 0.3350512821420176_f64 * t18418 - 0.1675256410710088_f64 * t18420 - 0.1675256410710088_f64 * t18422 + 0.3350512821420176_f64 * t18424 + 0.1675256410710088_f64 * t18426 + 0.1675256410710088_f64 * t18428 + 0.1675256410710088_f64 * t18430 + 0.1675256410710088_f64 * t18432 + 0.3350512821420176_f64 * t18434 - 0.0837628205355044_f64 * t84 * t18437;
    t18440
}
