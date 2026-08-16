//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 404/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk404(t1672: f64, t538: f64, t132: f64, t242: f64, t142: f64, t550: f64, t546: f64, t1684: f64, t1735: f64, t1666: f64, t1669: f64, t1905: f64, t309: f64, t633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2019 = 0.7380249726277691_f64 * t538 * t1672;
    let t2020 = t242 * t132;
    let t2021 = t142 * t2020;
    let t2023 = 2.3693919160612835_f64 * t550 * t2021;
    let t2025 = 0.8091720650647759_f64 * t546 * t1672;
    let t2026 = 0.10237773105191754_f64 * t1684;
    let t2027 = 0.03412591035063918_f64 * t1735;
    let t2029 = 0.04991874779241519_f64 * t1666;
    let t2030 = 0.01233429741534199_f64 * t1669;
    let t2032 = t309 * t1905 * t633;
    (t2019, t2021, t2023, t2025, t2026, t2027, t2029, t2030, t2032)
}
